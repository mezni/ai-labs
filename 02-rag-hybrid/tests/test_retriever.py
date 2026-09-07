"""Tests for Retriever (RRF fusion over BM25 + vector search)."""
import pytest

from src.knowledge.retriever import Retriever

LEAVE_VEC = (1.0, 0.0, 0.0)
ACCESS_VEC = (0.0, 1.0, 0.0)
BACKUP_VEC = (0.0, 0.0, 1.0)
VEC_BY_CONTENT = {
    "annual leave policy allows thirty days": list(LEAVE_VEC),
    "access control requires manager approval": list(ACCESS_VEC),
    "data backup completes every night": list(BACKUP_VEC),
}
CHUNKS = [
    {
        "content": "annual leave policy allows thirty days",
        "document_id": "p1",
        "source": "hr/leave.md",
    },
    {
        "content": "access control requires manager approval",
        "document_id": "p2",
        "source": "it/access.md",
    },
    {
        "content": "data backup completes every night",
        "document_id": "p3",
        "source": "it/backup.md",
    },
]


class HotEmbedder:
    """Embedder mapping known contents/queries to one-hot vectors."""

    def __init__(self, by_content: dict[str, list[float]], by_query: dict[str, list[float]]):
        self._by_content = by_content
        self._by_query = by_query

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        return [self._by_content[text] for text in texts]

    def embed_text(self, text: str) -> list[float]:
        return self._by_query[text]


def _build_index(kb_factory, tmp_path, query: str, query_vec: list[float] | tuple):
    embedder = HotEmbedder(VEC_BY_CONTENT, {query: query_vec})
    kb = kb_factory(
        corpus_dir=tmp_path,
        embedder=embedder,
        persist_dir=str(tmp_path / "chroma"),
    )
    kb.build_index(CHUNKS)
    return kb, embedder


def test_retriever_rejects_empty_chunks():
    with pytest.raises(ValueError, match="chunks must not be empty"):
        Retriever(collection=object(), embedder=object(), chunks=[])


def test_tokenize_normalizes_text():
    assert Retriever._tokenize("ALPHA-beta 123!") == ["alpha", "beta", "123"]


def test_retrieve_ranks_by_rrf_fusion(kb_factory, tmp_path):
    kb, embedder = _build_index(kb_factory, tmp_path, "manager approval", ACCESS_VEC)

    retriever = Retriever(kb.collection, embedder, kb._chunks)
    results = retriever.retrieve("manager approval", top_k=3)

    assert [result["source"] for result in results] == [
        "it/access.md",
        "hr/leave.md",
        "it/backup.md",
    ]
    scores = [result["score"] for result in results]
    assert scores == sorted(scores, reverse=True)
    assert scores[0] == pytest.approx(2 / 61)


def test_retrieve_honors_top_k(kb_factory, tmp_path):
    kb, embedder = _build_index(kb_factory, tmp_path, "manager approval", ACCESS_VEC)

    retriever = Retriever(kb.collection, embedder, kb._chunks)
    results = retriever.retrieve("manager approval", top_k=1)

    assert len(results) == 1
    assert results[0]["source"] == "it/access.md"


def test_retrieve_falls_back_to_dense_hit_for_missing_chunk(kb_factory, tmp_path):
    kb, embedder = _build_index(kb_factory, tmp_path, "annual leave", LEAVE_VEC)
    sparse_chunks = [chunk for chunk in kb._chunks if chunk["chunk_id"] != "p1_chunk_001"]

    retriever = Retriever(kb.collection, embedder, sparse_chunks)
    results = retriever.retrieve("annual leave", top_k=3)

    p1 = [r for r in results if r["chunk_id"] == "p1_chunk_001"]
    assert len(p1) == 1
    assert p1[0]["source"] == "hr/leave.md"
    assert "score" in p1[0]
    scores = [result["score"] for result in results]
    assert scores == sorted(scores, reverse=True)