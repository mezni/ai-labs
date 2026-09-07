"""Shared test fakes and fixtures (no ChromaDB or model downloads)."""
import pytest

from src.knowledge.knowledge_base import KnowledgeBase


class FakeEmbedder:
    """Minimal embedder returning zero vectors; no model required."""

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        return [[0.0] * 3 for _ in texts]

    def embed_text(self, text: str) -> list[float]:
        return [0.0] * 3


class FakeCollection:
    """In-memory ChromaDB-like collection with upsert/query."""

    def __init__(self, entries: list[dict] | None = None) -> None:
        self._entries = entries or []

    def upsert(
        self,
        ids: list[str],
        documents: list[str],
        embeddings: list[list[float]],
        metadatas: list[dict],
    ) -> None:
        for chunk_id, content, vector, metadata in zip(
            ids, documents, embeddings, metadatas
        ):
            self._entries.append(
                {"chunk_id": chunk_id, "content": content, "vector": vector, **metadata}
            )

    def query(self, query_embeddings, n_results: int = 5, include=None):
        query = query_embeddings[0]

        def _dist(entry: dict) -> float:
            return 1.0 - sum(a * b for a, b in zip(query, entry["vector"]))

        top = sorted(self._entries, key=_dist)[:n_results]
        return {
            "ids": [[entry["chunk_id"] for entry in top]],
            "documents": [[entry["content"] for entry in top]],
            "metadatas": [
                [
                    {
                        "document_id": entry["document_id"],
                        "source": entry["source"],
                        "category": entry["category"],
                    }
                    for entry in top
                ]
            ],
            "distances": [[_dist(entry) for entry in top]],
        }


@pytest.fixture
def kb_factory(monkeypatch):
    """Build a KnowledgeBase without touching a real ChromaDB client."""
    monkeypatch.setattr(
        KnowledgeBase,
        "_open_collection",
        lambda self, persist_dir, reset=False: FakeCollection(),
    )

    def _make(corpus_dir=None, **kwargs):
        kwargs.setdefault("embedder", FakeEmbedder())
        kwargs.setdefault("persist_dir", "/tmp/fake-persist")
        if corpus_dir is not None:
            kwargs["corpus_dir"] = corpus_dir
        return KnowledgeBase(**kwargs)

    return _make