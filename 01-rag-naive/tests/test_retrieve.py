"""Tests for ``rag.retrieve`` semantic retrieval."""

import pytest
from pydantic import ValidationError

from rag.chunk import chunk_document
from rag.clean import clean_pages
from rag.extract import ExtractedDocument
from rag.index import (
    Indexer,
    IndexIncompatibleError,
    IndexManifest,
    IndexOperationError,
    ManifestStore,
)
from rag.retrieve import RetrievalResult, Retriever
from tests.test_index import FakeEmbedder, _document, _long_text, _settings

CHUNK_SIZE = 120
CHUNK_OVERLAP = 20


def _chunks(document):
    cleaned = ExtractedDocument(document_id=document.document_id, pages=clean_pages(document.pages))
    return chunk_document(cleaned, chunk_size=CHUNK_SIZE, overlap=CHUNK_OVERLAP)


def _roaming_text():
    return " ".join(f"roaming clause {i} text about charges" for i in range(60))


def _indexed(tmp_path):
    settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
    embedder = FakeEmbedder(dimension=3)
    indexer = Indexer(embedder, settings=settings)
    indexer.index_document(_document("billing_policy.pdf", text=f"Aurora {_long_text()}"), "h1")
    indexer.index_document(_document("roaming_policy.pdf", text=_roaming_text()), "h2")
    return settings, embedder


def test_search_returns_ranked_results_sorted_by_score(tmp_path):
    settings, embedder = _indexed(tmp_path)
    retriever = Retriever(embedder, settings=settings)
    target = _chunks(_document("billing_policy.pdf", text=f"Aurora {_long_text()}"))[-1]

    results = retriever.search(target.text, top_k=5)

    assert results
    scores = [result.score for result in results]
    assert scores == sorted(scores, reverse=True)
    top = results[0]
    assert top.score == pytest.approx(1.0)
    assert top.text == target.text
    assert top.chunk_id == target.chunk_id
    assert top.document_id == "billing_policy.pdf"
    assert (top.page_start, top.page_end) == (target.page_start, target.page_end)


def test_search_respects_top_k(tmp_path):
    settings, embedder = _indexed(tmp_path)
    retriever = Retriever(embedder, settings=settings)

    results = retriever.search(f"Aurora {_long_text()}", top_k=1)

    assert len(results) == 1


def test_search_defaults_to_settings_top_k(tmp_path):
    settings = _settings(tmp_path, top_k=2, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
    embedder = FakeEmbedder(dimension=3)
    indexer = Indexer(embedder, settings=settings)
    indexer.index_document(_document(text=f"Aurora {_long_text()}"), "h1")
    retriever = Retriever(embedder, settings=settings)

    results = retriever.search(f"Aurora {_long_text()}")

    assert len(results) == 2


def test_search_maps_vector_ids_to_metadata(tmp_path):
    settings, embedder = _indexed(tmp_path)
    retriever = Retriever(embedder, settings=settings)

    results = retriever.search(f"Aurora {_long_text()}", top_k=5)

    assert results
    for result in results:
        record = retriever.metadata[result.vector_id]
        assert result.chunk_id == record.chunk_id
        assert result.document_id == record.document_id
        assert result.source == record.source
        assert result.text == record.text


def test_search_empty_index_returns_empty(tmp_path):
    settings = _settings(tmp_path, chunk_size=CHUNK_SIZE, chunk_overlap=CHUNK_OVERLAP)
    embedder = FakeEmbedder(dimension=3)
    indexer = Indexer(embedder, settings=settings)
    indexer.index_document(_document(text=f"Aurora {_long_text()}"), "h1")
    indexer.remove_document("billing_policy.pdf")
    retriever = Retriever(embedder, settings=settings)

    results = retriever.search(f"Aurora {_long_text()}")

    assert results == []


def test_search_rejects_non_positive_top_k(tmp_path):
    settings, embedder = _indexed(tmp_path)
    retriever = Retriever(embedder, settings=settings)

    with pytest.raises(ValueError, match="top_k"):
        retriever.search("question", top_k=0)


def test_retriever_requires_manifest(tmp_path):
    settings = _settings(tmp_path)

    with pytest.raises(IndexOperationError, match="index_manifest"):
        Retriever(FakeEmbedder(dimension=3), settings=settings)


def test_retriever_requires_index_file(tmp_path):
    settings = _settings(tmp_path)
    ManifestStore(settings.manifest_path).save(
        IndexManifest(embedding_model="model-a", embedding_dimension=3)
    )

    with pytest.raises(IndexOperationError, match="faiss.index"):
        Retriever(FakeEmbedder(dimension=3, model="model-a"), settings=settings)


def test_retriever_rejects_model_mismatch(tmp_path):
    settings, _ = _indexed(tmp_path)

    with pytest.raises(IndexIncompatibleError, match="embedding model"):
        Retriever(FakeEmbedder(dimension=3, model="openai/other"), settings=settings)


def test_search_rejects_query_dimension_mismatch(tmp_path):
    settings, _ = _indexed(tmp_path)
    retriever = Retriever(FakeEmbedder(dimension=5), settings=settings)

    with pytest.raises(IndexOperationError, match="query dimension"):
        retriever.search("question")


def test_retrieval_result_model_dump_shape():
    result = RetrievalResult(
        vector_id=1025,
        score=0.87,
        document_id="billing_policy.pdf",
        chunk_id="billing_policy.pdf::chunk::18",
        source="billing_policy.pdf",
        page_start=12,
        page_end=12,
        text="Refund requests are accepted within 30 days.",
    )

    data = result.model_dump(mode="json")

    assert set(data) == {
        "vector_id",
        "score",
        "document_id",
        "chunk_id",
        "source",
        "page_start",
        "page_end",
        "text",
    }
    assert data["vector_id"] == 1025
    assert data["score"] == 0.87
    assert data["chunk_id"] == "billing_policy.pdf::chunk::18"


def test_retrieval_result_rejects_invalid_pages():
    with pytest.raises(ValidationError):
        RetrievalResult(
            vector_id=1,
            score=0.5,
            document_id="d.pdf",
            chunk_id="d.pdf::chunk::0",
            source="d.pdf",
            page_start=0,
            page_end=1,
            text="text",
        )
