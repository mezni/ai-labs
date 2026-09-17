"""RAG retrieval over the FAISS-backed KnowledgeBase.

Ported from Bootcamp 3's src/knowledge/retriever.py.txt (semantic retrieval using
FAISS) and extended with the hybrid pipeline configured in config/llm_config.yaml
under retrieval.method:

    - "bm25+embedding+rerank" : BM25Okapi sparse + FAISS dense candidates, merged,
                                then re-scored with CrossEncoder("BAAI/bge-reranker-base")
    - "embedding"             : FAISS dense retrieval only (Bootcamp behaviour)

retrieve() always returns the same hit shape:
    {content, source, category, chunk_id, score}
"""
from typing import Any, Dict, List, Optional

import numpy as np

from src.knowledge.embeddings import BM25Index, EmbeddingGenerator, Reranker
from src.knowledge.knowledge_base import KnowledgeBase
from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

CONFIG = load_yaml_config("config/llm_config.yaml")


class Retriever:
    """Hybrid RAG retrieval over a FAISS-backed KnowledgeBase."""

    def __init__(self, knowledge_base: KnowledgeBase):
        self.kb = knowledge_base
        self.embedding_generator = EmbeddingGenerator()
        self.sparse_index = None
        self.reranker = None

        retrieval = CONFIG.get("retrieval", {})
        self.method = retrieval.get("method", "bm25+embedding+rerank")
        self.sparse_top_k = retrieval.get("sparse_retriever", {}).get("top_k", 15)
        self.dense_top_k = retrieval.get("dense_retriever", {}).get("top_k", 15)
        self.rerank_top_k = retrieval.get("reranker", {}).get("top_k", 8)

        if self.method != "embedding" and self.kb.documents and self.sparse_index is None:
            self.build_sparse_index(self.kb.documents)

    def build_sparse_index(self, documents: List[Dict[str, Any]]):
        """Tokenize `documents` into a BM25Okapi sparse index."""
        self.sparse_index = BM25Index().build(documents)
        logger.info("Built sparse index over %d documents", len(documents))

    def retrieve_sparse(self, query: str, top_k: Optional[int] = None) -> List[Dict[str, Any]]:
        """Retrieve candidates scored by BM25Okapi over the sparse index."""
        if self.sparse_index is None:
            logger.warning("Sparse index not built; call build_sparse_index() first")
            return []
        return [
            self._hit(doc, float(doc["score"]))
            for doc in self.sparse_index.search(query, top_k or self.sparse_top_k)
        ]

    def retrieve_dense(self, query: str, top_k: Optional[int] = None) -> List[Dict[str, Any]]:
        """Retrieve candidates scored by FAISS IndexFlatL2 over the embeddings."""
        if not self.kb.index or not self.kb.documents:
            return []

        query_embedding = self.embedding_generator.generate_embedding(query)
        query_vector = np.array([query_embedding]).astype("float32")

        distances, indices = self.kb.index.search(query_vector, top_k or self.dense_top_k)

        results = []
        for dist, idx in zip(distances[0], indices[0]):
            if idx < len(self.kb.documents):
                score = 1 / (1 + dist)
                results.append(self._hit(self.kb.documents[idx], score))
        return results

    def _hit(self, doc: Dict[str, Any], score: float) -> Dict[str, Any]:
        """Normalize a knowledge-base document into a retrieval hit."""
        hit = {
            "content": doc["content"],
            "source": doc["source"],
            "category": doc["category"],
            "score": float(score),
        }
        if "chunk_id" in doc:
            hit["chunk_id"] = doc["chunk_id"]
        return hit

    def _merge(self, *result_lists: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Deduplicate candidates by (source, chunk_id), keeping the best score."""
        merged: Dict[tuple, Dict[str, Any]] = {}
        for results in result_lists:
            for hit in results:
                key = (hit["source"], hit.get("chunk_id"))
                if key not in merged or hit["score"] > merged[key]["score"]:
                    merged[key] = hit
        return list(merged.values())

    def retrieve(
        self,
        query: str,
        top_k: Optional[int] = None,
        threshold: float = 0.0,
    ) -> List[Dict[str, Any]]:
        """Retrieve the documents most relevant to `query`.

        With the default hybrid method the sparse and dense candidate lists are
        merged, deduplicated by (source, chunk_id), and re-scored by the
        cross-encoder reranker. `top_k` defaults to retrieval.reranker.top_k.
        """
        if self.method == "embedding":
            return [
                hit for hit in self.retrieve_dense(query, top_k)
                if hit["score"] >= threshold
            ]

        candidates = self._merge(self.retrieve_sparse(query), self.retrieve_dense(query))
        if not candidates:
            return []

        if self.reranker is None:
            self.reranker = Reranker()

        reranked = self.reranker.rerank(query, candidates, top_k or self.rerank_top_k)
        return [hit for hit in reranked if hit["score"] >= threshold]