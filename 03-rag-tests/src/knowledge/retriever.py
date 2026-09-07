"""Hybrid retriever fusing BM25 (sparse) and FAISS (dense) search via RRF."""

from __future__ import annotations

from typing import Any

from src.knowledge.embeddings import BM25Index, Reranker
from src.knowledge.knowledge_base import Chunk, KnowledgeBase
from src.utils import get_logger

logger = get_logger(__name__)

RRF_CONSTANT = 60.0


class Retriever:
    """Retrieve top-k chunks for a query over the FAISS-backed KnowledgeBase.

    Dense candidates come from FAISS (via KnowledgeBase.search), sparse ones
    from BM25, and the two rankings are fused with Reciprocal Rank Fusion
    (RRF). Optionally, a cross-encoder Reranker re-scores the fused
    candidates for higher precision.
    """

    def __init__(
        self,
        knowledge_base: KnowledgeBase,
        sparse_top_k: int = 15,
        dense_top_k: int = 15,
        reranker: Reranker | None = None,
    ) -> None:
        chunks = knowledge_base.chunks
        if not chunks:
            raise ValueError("KnowledgeBase has no chunks to retrieve over")
        self.kb = knowledge_base
        self.reranker = reranker
        self.sparse_top_k = sparse_top_k
        self.dense_top_k = dense_top_k
        self._chunk_ids = [chunk["chunk_id"] for chunk in chunks]
        self._chunks_by_id: dict[str, Chunk] = {
            chunk["chunk_id"]: chunk for chunk in chunks
        }
        self._bm25 = BM25Index().add_documents(
            [chunk["content"] for chunk in chunks]
        )
        logger.info(
            "Retriever ready with %d chunks (sparse_top_k=%d, dense_top_k=%d)",
            len(chunks),
            sparse_top_k,
            dense_top_k,
        )

    def retrieve(self, query: str, top_k: int = 5) -> list[Chunk]:
        """Return the top-k chunks for the query, ranked by RRF fusion."""
        if not query.strip():
            raise ValueError("query must not be empty")

        sparse_ranks = self._sparse_candidates(query)
        dense_hits = self._dense_candidates(query)
        dense_by_id = {hit["chunk_id"]: hit for hit in dense_hits}
        rrf = self._fuse(dense_hits, sparse_ranks)
        candidates: list[Chunk] = []
        for chunk_id, score in sorted(rrf.items(), key=lambda item: item[1], reverse=True):
            chunk = self._chunks_by_id.get(chunk_id) or dense_by_id.get(chunk_id)
            if chunk is None:
                logger.warning("Dropping unknown chunk %r from results", chunk_id)
                continue
            copy = dict(chunk)
            copy["score"] = score
            dense_hit = dense_by_id.get(chunk_id)
            if dense_hit is not None:
                copy["dense_similarity"] = dense_hit.get("score")
            candidates.append(copy)

        if self.reranker is not None:
            return self._rerank_candidates(query, candidates[: top_k * 3], top_k)
        return candidates[:top_k]

    # -- ranking stages -------------------------------------------------------
    def _sparse_candidates(self, query: str) -> list[int]:
        """Indices of the top sparse_top_k chunks by BM25 score."""
        scores = self._bm25.get_scores(query)
        ranked = sorted(range(len(self._chunk_ids)), key=lambda i: scores[i], reverse=True)
        hits = [idx for idx in ranked if scores[idx] > 0][: self.sparse_top_k]
        logger.debug("Sparse candidates: %d", len(hits))
        return hits

    def _dense_candidates(self, query: str, top_k: int | None = None) -> list[Chunk]:
        """Chunks from FAISS vector search, by descending similarity."""
        hits = self.kb.search(query, top_k or self.dense_top_k)
        logger.debug("Dense candidates: %d", len(hits))
        return hits

    def _fuse(self, dense_hits: list[Chunk], sparse_ranks: list[int]) -> dict[str, float]:
        """Reciprocal Rank Fusion of dense (rank-order) and sparse hits."""
        rrf: dict[str, float] = {}
        for rank, hit in enumerate(dense_hits, start=1):
            rrf[hit["chunk_id"]] = rrf.get(hit["chunk_id"], 0.0) + 1.0 / (RRF_CONSTANT + rank)
        for rank, idx in enumerate(sparse_ranks, start=1):
            chunk_id = self._chunk_ids[idx]
            rrf[chunk_id] = rrf.get(chunk_id, 0.0) + 1.0 / (RRF_CONSTANT + rank)
        return rrf

    def _rerank_candidates(
        self,
        query: str,
        candidates: list[Chunk],
        top_k: int,
    ) -> list[Chunk]:
        """Cross-encoder re-score of the fused candidates, if a reranker is set."""
        if not candidates:
            return []
        documents = [c["content"] for c in candidates]
        reranked = self.reranker.rerank_with_scores(query, documents, top_n=top_k)
        scores_by_text = dict(reranked)
        results: list[Chunk] = []
        for candidate in candidates:
            if candidate["content"] in scores_by_text:
                copy = dict(candidate)
                copy["score"] = scores_by_text[candidate["content"]]
                results.append(copy)
        results.sort(key=lambda c: c["score"], reverse=True)
        logger.debug("Reranked to %d results", len(results))
        return results