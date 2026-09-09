from __future__ import annotations

import logging
from typing import Any

import chromadb
from pydantic import BaseModel

from src.config import Settings
from src.embeddings.embedder import Embedder, EmbeddingError
from src.indexing.registry import IndexRegistry

logger = logging.getLogger("rag.dataops.retrieval")


class RetrievalError(Exception):
    """Retrieval could not run (no current snapshot, model mismatch, ...)."""


class RetrievedChunk(BaseModel):
    """One top-k result with its lineage metadata (PROJECT.md §14/§8)."""

    chunk_id: str
    text: str
    score: float
    lineage: dict[str, Any]


class Retriever:
    """Embed the query and search the current versioned Chroma collection."""

    def __init__(self, settings: Settings, embedder: Embedder):
        self.settings = settings
        self.embedder = embedder
        self._registry = IndexRegistry().load(settings.index_registry)

    @property
    def registry(self) -> IndexRegistry:
        return self._registry

    def reload(self) -> None:
        self._registry = IndexRegistry().load(self.settings.index_registry)

    def retrieve(self, query: str, top_k: int | None = None) -> list[RetrievedChunk]:
        registry = IndexRegistry().load(self.settings.index_registry)
        current = registry.current()
        if current is None:
            raise RetrievalError("No current index version; run `rag-dataops index` first")
        if current.embedding_model != self.embedder.model:
            raise RetrievalError(
                f"Query embedding model {self.embedder.model!r} does not match the "
                f"registered snapshot model {current.embedding_model!r} (version "
                f"{registry.current_version})"
            )
        if not self.settings.chroma_versions_dir.joinpath(
            registry.current_version or ""
        ).is_dir():
            raise RetrievalError(
                f"Snapshot for version {registry.current_version} missing on disk"
            )
        vector = self._embed_query(query)
        collection = self._current_collection(registry)
        result = collection.query(
            query_embeddings=[vector],
            n_results=top_k or self.settings.top_k,
            include=["metadatas", "documents", "distances"],
        )
        return self._to_chunks(result)

    def _embed_query(self, query: str) -> list[float]:
        try:
            return self.embedder.embed([query])[0]
        except EmbeddingError as exc:
            raise RetrievalError(f"Query embedding failed: {exc}") from exc

    def _current_collection(self, registry: IndexRegistry) -> Any:
        current = registry.current()
        path = self.settings.chroma_versions_dir / (registry.current_version or "")
        client = chromadb.PersistentClient(path=str(path))
        return client.get_collection(current.collection_name)

    def _to_chunks(self, result: Any) -> list[RetrievedChunk]:
        chunk_ids = result["ids"][0] if result["ids"] else []
        documents = result["documents"][0] if result["documents"] else []
        metadatas = result["metadatas"][0] if result["metadatas"] else []
        distances = result["distances"][0] if result["distances"] else []
        chunks: list[RetrievedChunk] = []
        for chunk_id, text, metadata, distance in zip(
            chunk_ids, documents, metadatas, distances, strict=False
        ):
            lineage = {k: v for k, v in (metadata or {}).items() if v is not None}
            # Chroma returns distances (lower is closer); expose as a score.
            score = round(1.0 / (1.0 + float(distance)), 6)
            chunks.append(
                RetrievedChunk(
                    chunk_id=chunk_id,
                    text=text or "",
                    score=score,
                    lineage=lineage,
                )
            )
        return chunks