"""Hybrid retriever fusing BM25 (sparse) and vector (dense) search via RRF."""

from __future__ import annotations

import re
from typing import Any

from rank_bm25 import BM25Okapi

from src.knowledge.embeddings import EmbeddingGenerator
from src.knowledge.knowledge_base import Chunk
from src.utils.logger import get_logger

logger = get_logger(__name__)


class Retriever:
    """Hybrid retriever fusing BM25 (sparse) and vector (dense) search via RRF."""

    def __init__(
        self,
        collection: Any,
        embedder: EmbeddingGenerator,
        chunks: list[Chunk],
        sparse_top_k: int = 15,
        dense_top_k: int = 15,
    ) -> None:
        self.collection = collection
        self.embedder = embedder
        self.sparse_top_k = sparse_top_k
        self.dense_top_k = dense_top_k
        if not chunks:
            raise ValueError("chunks must not be empty")
        self._chunk_ids = [chunk["chunk_id"] for chunk in chunks]
        self._chunks_by_id = {chunk["chunk_id"]: chunk for chunk in chunks}
        self._bm25 = self._build_bm25(chunks)
        logger.info(
            "Retriever ready with %d chunks (sparse_top_k=%d, dense_top_k=%d)",
            len(chunks),
            sparse_top_k,
            dense_top_k,
        )

    @staticmethod
    def _tokenize(text: str) -> list[str]:
        return re.findall(r"[a-z0-9]+", text.lower())

    def _build_bm25(self, chunks: list[Chunk]) -> BM25Okapi:
        return BM25Okapi([self._tokenize(chunk["content"]) for chunk in chunks])

    def _dense_query(self, text: str, top_k: int) -> list[Chunk]:
        """Semantic search over the vector collection."""
        vector = self.embedder.embed_text(text)
        result = self.collection.query(
            query_embeddings=[vector],
            n_results=top_k,
            include=["documents", "metadatas", "distances"],
        )
        hits: list[Chunk] = []
        for chunk_id, doc, metadata, distance in zip(
            result["ids"][0],
            result["documents"][0],
            result["metadatas"][0],
            result["distances"][0],
        ):
            hits.append(
                {
                    "chunk_id": chunk_id,
                    "content": doc,
                    "document_id": metadata.get("document_id", ""),
                    "source": metadata.get("source", ""),
                    "category": metadata.get("category", ""),
                    "score": 1.0 - distance,
                }
            )
        return hits

    def retrieve(self, query: str, top_k: int = 5) -> list[Chunk]:
        """Return the top-k chunks for the query, ranked by RRF fusion."""
        tokens = self._tokenize(query)
        sparse_scores = self._bm25.get_scores(tokens)
        sparse_ranks = sorted(
            range(len(self._chunk_ids)),
            key=lambda i: sparse_scores[i],
            reverse=True,
        )[: self.sparse_top_k]
        logger.debug("Sparse top_k=%d candidates", len(sparse_ranks))

        dense_hits = self._dense_query(query, self.dense_top_k)
        logger.debug("Dense top_k=%d hits", len(dense_hits))
        dense_hits_by_id = {hit["chunk_id"]: hit for hit in dense_hits}

        rrf_scores: dict[str, float] = {}
        for rank, hit in enumerate(dense_hits, start=1):
            chunk_id = hit["chunk_id"]
            rrf_scores[chunk_id] = rrf_scores.get(chunk_id, 0.0) + 1.0 / (60 + rank)
        for rank, idx in enumerate(sparse_ranks, start=1):
            chunk_id = self._chunk_ids[idx]
            rrf_scores[chunk_id] = rrf_scores.get(chunk_id, 0.0) + 1.0 / (60 + rank)

        results: list[Chunk] = []
        for chunk_id, score in sorted(
            rrf_scores.items(), key=lambda item: item[1], reverse=True
        )[:top_k]:
            chunk = self._chunks_by_id.get(chunk_id)
            if chunk is None:
                chunk = dense_hits_by_id.get(chunk_id)
            if chunk is None:
                logger.warning("Dropping unknown chunk %r from results", chunk_id)
                continue
            copy = dict(chunk)
            copy["score"] = score
            results.append(copy)

        logger.info(
            "Retrieved %d results for %r (from %d sparse + %d dense candidates)",
            len(results),
            query,
            len(sparse_ranks),
            len(dense_hits),
        )
        return results