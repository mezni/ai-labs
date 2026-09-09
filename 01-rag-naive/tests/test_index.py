"""Tests for ``rag.index`` FAISS indexing, metadata, and end-to-end indexing."""

import hashlib

import faiss
import numpy as np
import pytest

from rag.chunk import chunk_document
from rag.config import Settings
from rag.embed import Embedding
from rag.extract import ExtractedDocument, ExtractedPage, extract_pdf
from rag.index import (
    IndexConsistencyError,
    Indexer,
    IndexIncompatibleError,
    IndexManifest,
    IndexOperationError,
    ManifestStore,
    MetadataRecord,
    MetadataStore,
    VectorIndex,
    validate_consistency,
)
from rag.state import Change, DocumentState, detect_changes, hash_documents
from tests.pdf_factory import build_pdf


def _settings(tmp_path, *, chunk_size=400, chunk_overlap=100, **overrides):
    values = {
        "chunk_size": chunk_size,
        "chunk_overlap": chunk_overlap,
        "openrouter_api_key": "sk-test",
        "openrouter_embedding_model": "openai/text-embedding-3-small",
        "data_dir": tmp_path / "data",
        "index_path": tmp_path / "indexes" / "faiss.index",
        "metadata_path": tmp_path / "indexes" / "metadata.json",
        "document_state_path": tmp_path / "indexes" / "document_state.json",
        "manifest_path": tmp_path / "indexes" / "index_manifest.json",
    }
    values.update(overrides)
    return Settings(**values)


def _seed_for(text):
    return int.from_bytes(hashlib.sha256(text.encode("utf-8")).digest()[:8], "little")


class FakeEmbedder:
    """Deterministic embedder: identical text in, identical vector out."""

    def __init__(self, model="openai/text-embedding-3-small", dimension=3):
        self.model = model
        self.dimension = dimension
        self.calls = 0

    def embed_texts(self, texts, *, expected_dimension=None):
        self.calls += 1
        return [
            Embedding(index=i, text=text, vector=self._vector(text)) for i, text in enumerate(texts)
        ]

    def _vector(self, text):
        rng = np.random.default_rng(_seed_for(text))
        return rng.uniform(-1.0, 1.0, size=self.dimension).astype(float).tolist()


def _document(document_id="billing_policy.pdf", text="billing policy text"):
    return ExtractedDocument(
        document_id=document_id, pages=[ExtractedPage(page_number=1, text=text)]
    )


def _long_text():
    return " ".join(f"policy {i} clause number {i} governs refunds" for i in range(80))


def _write_pdf(directory, name, pages):
    path = directory / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(build_pdf(pages))
    return path


class TestVectorIndex:
    def test_create_add_search_returns_explicit_ids(self):
        index = VectorIndex.create(3)

        index.add([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], [1000, 1001, 1002])

        ids, scores = index.search([1.0, 0.0, 0.0], 3)

        assert index.ntotal == 3
        assert index.dimension == 3
        assert ids[0] == 1000
        assert set(ids) == {1000, 1001, 1002}
        assert scores[0] == pytest.approx(1.0)
        assert scores[1] == pytest.approx(0.0)

    def test_remove_removes_ids(self):
        index = VectorIndex.create(3)
        index.add([[1.0, 0, 0], [0, 1.0, 0], [0, 0, 1.0]], [1000, 1001, 1002])

        index.remove([1001])

        assert index.ids() == {1000, 1002}
        ids, _ = index.search([0.0, 1.0, 0.0], 3)
        assert 1001 not in ids

    def test_save_load_roundtrip_preserves_ids(self, tmp_path):
        path = tmp_path / "faiss.index"
        index = VectorIndex.create(3)
        index.add([[1.0, 0, 0], [0, 1.0, 0]], [1000, 1001])

        index.save(path)
        loaded = VectorIndex.load(path)

        assert loaded.ntotal == 2
        assert loaded.ids() == {1000, 1001}
        ids, _ = loaded.search([1.0, 0, 0], 2)
        assert ids == [1000, 1001]

    def test_add_rejects_dimension_mismatch(self):
        index = VectorIndex.create(3)

        with pytest.raises(IndexOperationError, match="dimension"):
            index.add([[1.0, 0.0]], [1000])

    def test_search_rejects_dimension_mismatch(self):
        index = VectorIndex.create(3)
        index.add([[1.0, 0.0, 0.0]], [1000])

        with pytest.raises(IndexOperationError, match="dimension"):
            index.search([1.0, 0.0], 1)

    def test_load_rejects_non_idmap_index(self, tmp_path):
        path = tmp_path / "plain.index"
        faiss.write_index(faiss.IndexFlatIP(3), str(path))

        with pytest.raises(IndexOperationError, match="IndexIDMap2"):
            VectorIndex.load(path)


class TestStores:
    def test_metadata_store_roundtrip(self, tmp_path):
        path = tmp_path / "metadata.json"
        store = MetadataStore(path)
        records = {
            1000: MetadataRecord(
                vector_id=1000,
                chunk_id="billing_policy.pdf::chunk::0",
                document_id="billing_policy.pdf",
                source="billing_policy.pdf",
                chunk_index=0,
                page_start=1,
                page_end=1,
                text="refund policy",
            )
        }

        store.save(records)
        loaded = MetadataStore(path).load()

        assert loaded == records

    def test_metadata_store_load_missing_returns_empty(self, tmp_path):
        assert MetadataStore(tmp_path / "missing.json").load() == {}

    def test_manifest_store_roundtrip(self, tmp_path):
        path = tmp_path / "index_manifest.json"
        store = ManifestStore(path)
        manifest = IndexManifest(
            embedding_model="model-a", embedding_dimension=3, next_vector_id=10
        )

        store.save(manifest)
        loaded = ManifestStore(path).load()

        assert loaded == manifest

    def test_manifest_store_load_missing_returns_none(self, tmp_path):
        assert ManifestStore(tmp_path / "missing.json").load() is None

    def test_manifest_model_dump_shape(self):
        manifest = IndexManifest(embedding_model="model-a", embedding_dimension=3)

        data = manifest.model_dump(mode="json")

        assert set(data) == {
            "version",
            "embedding_model",
            "embedding_dimension",
            "similarity",
            "index_type",
            "next_vector_id",
            "vector_count",
        }
        assert data["similarity"] == "cosine"
        assert data["index_type"] == "IndexIDMap2(IndexFlatIP)"


class TestConsistency:
    def _consistent(
        self,
        *,
        index_ids=None,
        metadata_ids=None,
        state_ids=None,
        vector_count=None,
    ):
        index_ids = [1000, 1001] if index_ids is None else index_ids
        metadata_ids = [1000, 1001] if metadata_ids is None else metadata_ids
        state_ids = [1000, 1001] if state_ids is None else state_ids
        count = len(index_ids) if vector_count is None else vector_count

        index = VectorIndex.create(3)
        vectors = [[1.0, 0, 0], [0, 1.0, 0]] if len(index_ids) == 2 else [[1.0, 0, 0]]
        index.add(vectors, index_ids)

        metadata = {
            vector_id: MetadataRecord(
                vector_id=vector_id,
                chunk_id=f"doc.pdf::chunk::{i}",
                document_id="doc.pdf",
                source="doc.pdf",
                chunk_index=i,
                page_start=1,
                page_end=1,
                text="text",
            )
            for i, vector_id in enumerate(metadata_ids)
        }
        state = {
            "doc.pdf": DocumentState(hash="aa", vector_ids=state_ids, chunk_count=len(state_ids))
        }
        manifest = IndexManifest(
            embedding_model="model-a", embedding_dimension=3, vector_count=count
        )
        return index, metadata, state, manifest

    def test_validate_consistency_passes(self):
        index, metadata, state, manifest = self._consistent()

        validate_consistency(index, metadata, state, manifest)

    def test_validate_consistency_fails_on_index_metadata_mismatch(self):
        index, metadata, state, manifest = self._consistent(metadata_ids=[1000])

        with pytest.raises(IndexConsistencyError, match="metadata"):
            validate_consistency(index, metadata, state, manifest)

    def test_validate_consistency_fails_on_state_vector_missing_from_index(self):
        index, metadata, state, manifest = self._consistent(state_ids=[1000, 1001, 1002])

        with pytest.raises(IndexConsistencyError, match="missing from FAISS"):
            validate_consistency(index, metadata, state, manifest)

    def test_validate_consistency_fails_on_orphan_metadata(self):
        index, metadata, state, manifest = self._consistent()
        index.add([[0.0, 0.0, 1.0]], [2000])
        metadata[2000] = MetadataRecord(
            vector_id=2000,
            chunk_id="doc.pdf::chunk::2",
            document_id="doc.pdf",
            source="doc.pdf",
            chunk_index=2,
            page_start=1,
            page_end=1,
            text="orphan",
        )

        with pytest.raises(IndexConsistencyError, match="not listed"):
            validate_consistency(index, metadata, state, manifest)


class TestIndexer:
    def test_index_new_document_persists_everything(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        embedder = FakeEmbedder(dimension=3)
        indexer = Indexer(embedder, settings=settings)
        document = _document(text=_long_text())

        vector_ids = indexer.index_document(document, document_hash="aa")

        expected_chunks = len(chunk_document(document, chunk_size=120, overlap=20))
        assert len(vector_ids) == expected_chunks
        assert indexer.manifest.embedding_dimension == 3
        assert indexer.index.ntotal == expected_chunks
        assert len(indexer.metadata_records) == expected_chunks
        assert indexer.state["billing_policy.pdf"].hash == "aa"
        assert settings.manifest_path.is_file()
        assert settings.metadata_path.is_file()
        assert settings.document_state_path.is_file()
        assert not list(settings.index_path.parent.glob("*.tmp"))
        validate_consistency(
            indexer.index, indexer.metadata_records, indexer.state, indexer.manifest
        )

    def test_index_open_existing_manifest_is_consistent(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        Indexer(FakeEmbedder(dimension=3), settings=settings).index_document(
            _document(text=_long_text()), document_hash="aa"
        )

        reopen = Indexer(FakeEmbedder(dimension=3), settings=settings)

        assert reopen.manifest.embedding_dimension == 3
        assert reopen.index.ntotal == reopen.manifest.vector_count
        validate_consistency(reopen.index, reopen.metadata_records, reopen.state, reopen.manifest)

    def test_index_rejects_model_change_on_reopen(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        Indexer(FakeEmbedder(dimension=3, model="openai/a"), settings=settings).index_document(
            _document(text=_long_text()), document_hash="aa"
        )

        with pytest.raises(IndexIncompatibleError, match="embedding model"):
            Indexer(FakeEmbedder(dimension=3, model="openai/b"), settings=settings)

    def test_index_rejects_embedding_dimension_change(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        Indexer(FakeEmbedder(dimension=3), settings=settings).index_document(
            _document(text=_long_text()), document_hash="aa"
        )

        indexer = Indexer(FakeEmbedder(dimension=5), settings=settings)

        with pytest.raises(IndexIncompatibleError, match="dimension"):
            indexer.index_document(_document("new.pdf", text=_long_text()), document_hash="bb")

    def test_retrieval_roundtrip_returns_matching_chunk(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        embedder = FakeEmbedder(dimension=3)
        indexer = Indexer(embedder, settings=settings)
        document = _document(text=_long_text())
        indexer.index_document(document, document_hash="aa")

        chunks = chunk_document(document, chunk_size=120, overlap=20)
        query = embedder.embed_texts([chunks[0].text])[0].vector
        ids, scores = indexer.index.search(query, 1)

        found = indexer.metadata_records[ids[0]]
        assert found.text == chunks[0].text
        assert scores[0] == pytest.approx(1.0)

    def test_replace_document_swaps_vectors_safely(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        indexer = Indexer(FakeEmbedder(dimension=3), settings=settings)
        first = _document(text=f"version one {_long_text()}")
        second = _document(text=f"version two {_long_text()}")

        old_ids = indexer.index_document(first, document_hash="h1")
        new_ids = indexer.replace_document(second, document_hash="h2")

        assert set(old_ids) & set(new_ids) == set()
        assert indexer.state["billing_policy.pdf"].hash == "h2"
        assert set(old_ids) & indexer.index.ids() == set()
        assert set(new_ids) & indexer.index.ids() == set(new_ids)
        assert set(old_ids) & set(indexer.metadata_records) == set()

    def test_index_document_for_existing_routes_to_safe_replace(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        indexer = Indexer(FakeEmbedder(dimension=3), settings=settings)
        first = _document(text=f"alpha {_long_text()}")
        second = _document(text=f"beta {_long_text()}")

        old_ids = indexer.index_document(first, document_hash="h1")
        new_ids = indexer.index_document(second, document_hash="h2")

        assert set(old_ids) & set(new_ids) == set()
        assert set(old_ids) & indexer.index.ids() == set()
        assert indexer.state["billing_policy.pdf"].hash == "h2"

    def test_remove_document_cleans_up_stores(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        indexer = Indexer(FakeEmbedder(dimension=3), settings=settings)
        first_ids = indexer.index_document(_document("first.pdf", text=_long_text()), "h1")
        second_ids = indexer.index_document(_document("second.pdf", text=_long_text()), "h2")

        indexer.remove_document("first.pdf")

        assert "first.pdf" not in indexer.state
        assert set(first_ids) & indexer.index.ids() == set()
        assert set(first_ids) & set(indexer.metadata_records) == set()
        assert set(second_ids) & indexer.index.ids() == set(second_ids)
        validate_consistency(
            indexer.index, indexer.metadata_records, indexer.state, indexer.manifest
        )

    def test_remove_unknown_document_is_noop(self, tmp_path):
        settings = _settings(tmp_path)
        indexer = Indexer(FakeEmbedder(dimension=3), settings=settings)
        vector_ids = indexer.index_document(_document(text=_long_text()), "aa")

        indexer.remove_document("ghost.pdf")

        assert indexer.state == {"billing_policy.pdf": indexer.state["billing_policy.pdf"]}
        assert indexer.index.ntotal == len(vector_ids)


class TestIncremental:
    def test_unchanged_run_makes_zero_embedding_calls(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        _write_pdf(settings.data_dir, "billing_policy.pdf", [f"policy {i} text" for i in range(3)])

        hashes = hash_documents(settings.data_dir)
        assert detect_changes(hashes, {}) == {"billing_policy.pdf": Change.NEW}
        Indexer(FakeEmbedder(dimension=3), settings=settings).index_document(
            extract_pdf(settings.data_dir / "billing_policy.pdf"), hashes["billing_policy.pdf"]
        )

        second_embedder = FakeEmbedder(dimension=3)
        second_run = Indexer(second_embedder, settings=settings)
        second_hashes = hash_documents(settings.data_dir)
        changes = detect_changes(second_hashes, second_run.state)

        assert changes == {"billing_policy.pdf": Change.UNCHANGED}
        assert second_embedder.calls == 0

    def test_only_changed_document_is_reprocessed(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        first = _write_pdf(
            settings.data_dir, "billing_policy.pdf", [f"billing {i}" for i in range(3)]
        )
        _write_pdf(settings.data_dir, "roaming_policy.pdf", [f"roaming {i}" for i in range(3)])

        initial_hashes = hash_documents(settings.data_dir)
        initial = Indexer(FakeEmbedder(dimension=3), settings=settings)
        initial.index_document(extract_pdf(first), initial_hashes["billing_policy.pdf"])
        initial.index_document(
            extract_pdf(settings.data_dir / "roaming_policy.pdf"),
            initial_hashes["roaming_policy.pdf"],
        )
        unchanged_ids = initial.state["roaming_policy.pdf"].vector_ids

        _write_pdf(
            settings.data_dir, "billing_policy.pdf", [f"revised billing {i}" for i in range(4)]
        )

        second_embedder = FakeEmbedder(dimension=3)
        second = Indexer(second_embedder, settings=settings)
        second_hashes = hash_documents(settings.data_dir)
        changes = detect_changes(second_hashes, second.state)

        assert changes["billing_policy.pdf"] == Change.CHANGED
        assert changes["roaming_policy.pdf"] == Change.UNCHANGED

        second.index_document(
            extract_pdf(settings.data_dir / "billing_policy.pdf"),
            second_hashes["billing_policy.pdf"],
        )

        assert second_embedder.calls == 1
        assert set(unchanged_ids) & second.index.ids() == set(unchanged_ids)
        assert set(unchanged_ids) <= set(second.metadata_records)
        validate_consistency(second.index, second.metadata_records, second.state, second.manifest)

    def test_deleted_document_removed_on_reopen(self, tmp_path):
        settings = _settings(tmp_path, chunk_size=120, chunk_overlap=20)
        pdf = _write_pdf(settings.data_dir, "billing_policy.pdf", ["billing policy"])
        initial_hashes = hash_documents(settings.data_dir)
        initial = Indexer(FakeEmbedder(dimension=3), settings=settings)
        initial.index_document(extract_pdf(pdf), initial_hashes["billing_policy.pdf"])
        total_before = initial.index.ntotal

        pdf.unlink()

        second = Indexer(FakeEmbedder(dimension=3), settings=settings)
        changes = detect_changes(hash_documents(settings.data_dir), second.state)

        assert changes == {"billing_policy.pdf": Change.DELETED}
        second.remove_document("billing_policy.pdf")

        assert second.index.ntotal == 0
        assert total_before > 0
        assert second.state == {}
        assert second.metadata_records == {}
