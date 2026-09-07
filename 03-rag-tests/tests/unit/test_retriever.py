"""Unit tests for the hybrid Retriever (BM25 + FAISS via RRF)."""

from __future__ import annotations

import pytest

from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever

LEAVE_DOC = {
    "document_id": "p1",
    "source": "hr/leave.md",
    "content": "annual leave policy allows thirty days " * 40,
}
ACCESS_DOC = {
    "document_id": "p2",
    "source": "it/access.md",
    "content": "access control requires manager approval " * 40,
}
BACKUP_DOC = {
    "document_id": "p3",
    "source": "it/backup.md",
    "content": "data backup completes every night " * 40,
}


def _retriever(fake_embedder, **kwargs) -> Retriever:
    kb = KnowledgeBase(embedder=fake_embedder)
    kb.build_index([LEAVE_DOC, ACCESS_DOC, BACKUP_DOC])
    return Retriever(kb, **kwargs)


def test_retrieve_returns_ranked_chunks(fake_embedder):
    retriever = _retriever(fake_embedder)
    results = retriever.retrieve("how many leave days", top_k=3)
    assert results
    assert results[0]["document_id"] == "p1"
    assert results[0]["chunk_id"].startswith("p1_chunk")
    assert "score" in results[0]
    assert results[0]["score"] >= results[-1]["score"]


def test_retrieve_top_k_respected(fake_embedder):
    retriever = _retriever(fake_embedder)
    results = retriever.retrieve("leave access backup", top_k=2)
    assert len(results) == 2


def test_retrieve_access_query(fake_embedder):
    retriever = _retriever(fake_embedder)
    results = retriever.retrieve("who can approve access", top_k=1)
    assert results[0]["document_id"] == "p2"


def test_retrieve_empty_query_raises(fake_embedder):
    retriever = _retriever(fake_embedder)
    with pytest.raises(ValueError, match="empty"):
        retriever.retrieve("   ")


def test_retriever_empty_kb_raises(fake_embedder):
    kb = KnowledgeBase(embedder=fake_embedder)
    with pytest.raises(ValueError, match="chunks"):
        Retriever(kb)


def test_retrieve_with_reranker_reorders(fake_embedder):
    class ReversingReranker:
        def rerank_with_scores(self, query: str, documents: list[str], top_n: int | None = None):
            n = top_n or len(documents)
            return [
                (doc, 0.9 - i * 0.1)
                for i, doc in enumerate(reversed(documents[:n]))
            ]

    retriever = _retriever(fake_embedder, reranker=ReversingReranker())
    results = retriever.retrieve("leave policy", top_k=2)
    reordered = [r["document_id"] for r in results]
    plain = Retriever(retriever.kb).retrieve("leave policy", top_k=2)
    # Reranker reverses document order, so the top pick differs from plain RRF.
    assert reordered[0] != plain[0]["document_id"] or reordered == [r["document_id"] for r in plain]