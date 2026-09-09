"""Tests for ``rag.main`` CLI orchestration across pipeline stages."""

import json
import logging

import pytest

from rag.chunk import chunk_document
from rag.clean import clean_pages
from rag.embed import EmbeddingError
from rag.extract import ExtractedDocument, extract_pdf
from rag.generate import GeneratedAnswer
from rag.index import Indexer, IndexOperationError, validate_consistency
from rag.main import configure_logging, main, run_ask, run_index, run_reindex, run_search
from tests.test_index import FakeEmbedder, _long_text, _settings, _write_pdf

CHUNK_SIZE = 120
CHUNK_OVERLAP = 20


def _chunks(document):
    cleaned = ExtractedDocument(document_id=document.document_id, pages=clean_pages(document.pages))
    return chunk_document(cleaned, chunk_size=CHUNK_SIZE, overlap=CHUNK_OVERLAP)


class StubGenerator:
    ANSWER = "Refunds are accepted within 30 days."

    def __init__(self):
        self.calls = 0

    def generate(self, question, results, *, system_prompt=None):
        self.calls += 1
        return GeneratedAnswer(answer=self.ANSWER, sources=list(results))


class TestConfigureLogging:
    def test_configure_logging_creates_log_file(self, tmp_path):
        settings = _settings(tmp_path)

        configure_logging(settings)

        assert settings.log_path.is_file()
        assert settings.log_path.parent.is_dir()

    def test_configure_logging_is_idempotent(self, tmp_path):
        settings = _settings(tmp_path)

        configure_logging(settings)
        configure_logging(settings)

        matches = [
            handler
            for handler in logging.getLogger().handlers
            if isinstance(handler, logging.FileHandler)
            and getattr(handler, "baseFilename", None) == str(settings.log_path)
        ]
        assert len(matches) == 1


class TestRunIndex:
    def test_run_index_persists_every_document(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"billing {i}" for i in range(3)])
        _write_pdf(settings.data_dir, "roaming_policy.pdf", ["roaming policy"])

        indexer = run_index(settings, embedder=FakeEmbedder(dimension=3))

        assert set(indexer.state) == {"billing_policy.pdf", "roaming_policy.pdf"}
        assert settings.manifest_path.is_file()
        assert settings.index_path.is_file()
        validate_consistency(
            indexer.index, indexer.metadata_records, indexer.state, indexer.manifest
        )

    def test_run_index_skips_unchanged_documents(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", ["billing policy"])

        run_index(settings, embedder=FakeEmbedder(dimension=3))
        fresh = FakeEmbedder(dimension=3)
        run_index(settings, embedder=fresh)

        assert fresh.calls == 0

    def test_run_index_replaces_only_changed_document(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"billing {i}" for i in range(3)])
        _write_pdf(settings.data_dir, "roaming_policy.pdf", [f"roaming {i}" for i in range(2)])

        first = run_index(settings, embedder=FakeEmbedder(dimension=3))
        unchanged_ids = first.state["roaming_policy.pdf"].vector_ids

        _write_pdf(
            settings.data_dir, "billing_policy.pdf", [f"revised billing {i}" for i in range(4)]
        )
        second_embedder = FakeEmbedder(dimension=3)

        second = run_index(settings, embedder=second_embedder)

        assert second_embedder.calls == 1
        assert second.state["billing_policy.pdf"].hash != first.state["billing_policy.pdf"].hash
        assert set(unchanged_ids) & second.index.ids() == set(unchanged_ids)

    def test_run_index_removes_deleted_document(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        pdf = _write_pdf(settings.data_dir, "billing_policy.pdf", ["billing policy"])
        first = run_index(settings, embedder=FakeEmbedder(dimension=3))
        assert first.index.ntotal > 0

        pdf.unlink()
        run_index(settings, embedder=FakeEmbedder(dimension=3))

        indexer = Indexer(FakeEmbedder(dimension=3), settings=settings)
        assert indexer.state == {}
        assert indexer.index.ntotal == 0
        assert indexer.metadata_records == {}

    def test_run_index_keeps_going_after_embedding_failure(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "bad.pdf", ["boom bad document"])
        _write_pdf(settings.data_dir, "good.pdf", ["good document text"])

        class SelectivelyFailing(FakeEmbedder):
            def embed_texts(self, texts, *, expected_dimension=None):
                if any("boom" in text for text in texts):
                    raise EmbeddingError("embedding refused")
                return super().embed_texts(texts, expected_dimension=expected_dimension)

        indexer = run_index(settings, embedder=SelectivelyFailing(dimension=3))

        assert "bad.pdf" not in indexer.state
        assert "good.pdf" in indexer.state

    def test_run_index_keeps_going_after_extraction_failure(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        settings.data_dir.mkdir(parents=True, exist_ok=True)
        (settings.data_dir / "broken.pdf").write_bytes(b"not a real pdf")
        _write_pdf(settings.data_dir, "good.pdf", ["good document text"])

        indexer = run_index(settings, embedder=FakeEmbedder(dimension=3))

        assert "broken.pdf" not in indexer.state
        assert "good.pdf" in indexer.state

    def test_run_index_stops_on_index_operation_failure(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "a.pdf", ["a document"])
        _write_pdf(settings.data_dir, "b.pdf", ["b document"])

        class CriticalFailure(FakeEmbedder):
            def embed_texts(self, texts, *, expected_dimension=None):
                raise IndexOperationError("index state corrupt")

        indexer = run_index(settings, embedder=CriticalFailure(dimension=3))

        assert indexer.state == {}


class TestRunSearch:
    def test_run_search_returns_ranked_results(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        embedder = FakeEmbedder(dimension=3)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"Aurora {_long_text()}"])
        run_index(settings, embedder=embedder)

        results = run_search(settings, f"Aurora {_long_text()}", embedder=embedder)

        assert results
        assert [{result.document_id} for result in results] == [
            {"billing_policy.pdf"} for _ in results
        ]
        scores = [result.score for result in results]
        assert scores == sorted(scores, reverse=True)

    def test_run_search_matches_exact_chunk(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        embedder = FakeEmbedder(dimension=3)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"Aurora {_long_text()}"])
        run_index(settings, embedder=embedder)

        extracted = extract_pdf(settings.data_dir / "billing_policy.pdf")
        chunks = _chunks(extracted)

        results = run_search(settings, chunks[0].text, embedder=embedder, top_k=5)

        assert results[0].score == pytest.approx(1.0)
        assert results[0].text == chunks[0].text


class TestRunAsk:
    def test_run_ask_generates_answer_with_sources(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        embedder = FakeEmbedder(dimension=3)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"Aurora {_long_text()}"])
        run_index(settings, embedder=embedder)
        generator = StubGenerator()

        answer = run_ask(settings, "Refund question", embedder=embedder, generator=generator)

        assert answer.answer == StubGenerator.ANSWER
        assert answer.sources
        assert generator.calls == 1

    def test_run_ask_respects_top_k(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        embedder = FakeEmbedder(dimension=3)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"Aurora {_long_text()}"])
        run_index(settings, embedder=embedder)

        answer = run_ask(
            settings,
            "Refund question",
            embedder=embedder,
            generator=StubGenerator(),
            top_k=1,
        )

        assert len(answer.sources) == 1


class TestRunReindex:
    def test_run_reindex_rebuilds_from_scratch(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", ["billing policy"])
        _write_pdf(settings.data_dir, "roaming_policy.pdf", ["roaming policy"])
        first = run_index(settings, embedder=FakeEmbedder(dimension=3))
        assert set(first.state) == {"billing_policy.pdf", "roaming_policy.pdf"}

        (settings.data_dir / "roaming_policy.pdf").unlink()

        second = run_reindex(settings, embedder=FakeEmbedder(dimension=3))

        assert set(second.state) == {"billing_policy.pdf"}
        assert settings.manifest_path.is_file()
        assert settings.index_path.is_file()
        validate_consistency(second.index, second.metadata_records, second.state, second.manifest)


class TestMainDispatch:
    def _patch_builders(self, monkeypatch, embedder=None):
        embedder = embedder or FakeEmbedder(dimension=3)
        generator = StubGenerator()
        monkeypatch.setattr("rag.main.build_embedder", lambda settings: embedder)
        monkeypatch.setattr("rag.main.build_generator", lambda settings: generator)
        return embedder, generator

    def test_main_index_subcommand(self, tmp_path, monkeypatch):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", ["billing policy"])
        self._patch_builders(monkeypatch)

        assert main(["index"], settings=settings) == 0
        assert settings.manifest_path.is_file()

    def test_main_reindex_subcommand(self, tmp_path, monkeypatch):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", ["billing policy"])
        self._patch_builders(monkeypatch)

        assert main(["reindex"], settings=settings) == 0
        assert settings.index_path.is_file()

    def test_main_search_prints_json_lines(self, tmp_path, monkeypatch, capsys):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"Aurora {_long_text()}"])
        embedder, _ = self._patch_builders(monkeypatch)
        run_index(settings, embedder=embedder)

        assert main(["search", f"Aurora {_long_text()}", "--top-k", "1"], settings=settings) == 0

        lines = capsys.readouterr().out.strip().splitlines()
        assert len(lines) == 1
        payload = json.loads(lines[0])
        assert payload["document_id"] == "billing_policy.pdf"

    def test_main_ask_prints_answer_and_sources(self, tmp_path, monkeypatch, capsys):
        settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"Aurora {_long_text()}"])
        embedder, _ = self._patch_builders(monkeypatch)
        run_index(settings, embedder=embedder)

        assert main(["ask", "Refund question", "--top-k", "1"], settings=settings) == 0

        lines = capsys.readouterr().out.strip().splitlines()
        assert len(lines) == 2
        assert lines[0] == StubGenerator.ANSWER
        assert json.loads(lines[1])["document_id"] == "billing_policy.pdf"
