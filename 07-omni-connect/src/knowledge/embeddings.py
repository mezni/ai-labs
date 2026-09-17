"""Local embedding & retrieval utilities for RAG over the knowledge base.

Rewrites the OpenAI-backed Bootcamp 3 EmbeddingGenerator (src/knowledge/
embeddings.py.txt) to run fully locally with:

    - dense embeddings : sentence-transformers  BAAI/bge-small-en-v1.5
    - sparse retrieval: rank_bm25             BM25Okapi
    - re-ranking       : sentence-transformers CrossEncoder
                         BAAI/bge-reranker-base
    - tensor ops       : torch (device selection, no_grad inference,
                         normalized embeddings)

Model names and retrieval settings are read from config/llm_config.yaml
(models.embedding, models.reranker, retrieval). Nothing is downloaded at
import time; models are loaded lazily on instantiation.
"""
import logging
import re
from typing import Any, Dict, List, Optional

import torch
from rank_bm25 import BM25Okapi
from sentence_transformers import CrossEncoder, SentenceTransformer

from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

CONFIG = load_yaml_config("config/llm_config.yaml")


def _device() -> str:
    if torch.cuda.is_available():
        return "cuda"
    if torch.backends.mps.is_available():
        return "mps"
    return "cpu"


def _to_list(vectors) -> List[float]:
    """Version-robust conversion of an encode()/predict() result to a list."""
    return vectors.tolist() if hasattr(vectors, "tolist") else list(vectors)


def _default_tokenizer(text: str) -> List[str]:
    """Lightweight whitespace/punctuation tokenizer for BM25."""
    return re.findall(r"[a-zA-Z0-9]+", text.lower())


class EmbeddingGenerator:
    """Dense embeddings from a local sentence-transformers model.

    The model comes from config/llm_config.yaml's models.embedding section by
    default (BAAI/bge-small-en-v1.5). Pass an explicit model name to override
    for a specific instance without editing the YAML file.
    """

    def __init__(
        self,
        model: Optional[str] = None,
        normalize: bool = True,
        seed: Optional[int] = None,
    ):
        embedding_config = CONFIG.get("models", {}).get("embedding", {})
        self.model_name = model or embedding_config.get("model_name", "BAAI/bge-small-en-v1.5")
        self.dimensions = embedding_config.get("dimensions", 384)
        self.normalize = normalize

        if seed is not None:
            torch.manual_seed(seed)

        logger.info("Loading embedding model %s on %s", self.model_name, _device())
        self.model = SentenceTransformer(self.model_name, device=_device())
        self.model.eval()

    def generate_embedding(self, text: str) -> List[float]:
        """Generate embedding for a single text."""
        return self.generate_embeddings([text])[0]

    def generate_embeddings(self, texts: List[str]) -> List[List[float]]:
        """Generate normalized embeddings for multiple texts (no_grad)."""
        with torch.no_grad():
            vectors = self.model.encode(
                texts,
                batch_size=32,
                normalize_embeddings=self.normalize,
                show_progress_bar=False,
            )
        return [_to_list(vector) for vector in vectors]


class BM25Index:
    """Sparse retrieval index over chunked documents, built with BM25Okapi.

    Pure-Python (rank_bm25) - no model weights involved, so it can be built
    even before the embedding model is downloaded.
    """

    def __init__(
        self,
        documents: Optional[List[Dict[str, Any]]] = None,
        tokenizer=None,
    ):
        self.documents: List[Dict[str, Any]] = documents or []
        self.tokenizer = tokenizer or _default_tokenizer
        self.bm25: Optional[BM25Okapi] = None

    def build(self, documents: List[Dict[str, Any]]) -> "BM25Index":
        """Tokenize `documents` and build the BM25Okapi index."""
        self.documents = documents
        corpus = [self.tokenizer(doc["content"]) for doc in documents]
        self.bm25 = BM25Okapi(corpus)
        logger.info("Built BM25 index over %d documents", len(documents))
        return self

    def search(self, query: str, top_k: int = 5) -> List[Dict[str, Any]]:
        """Return the top_k documents for `query`, scored by BM25."""
        if self.bm25 is None or not self.documents:
            return []
        scores = self.bm25.get_scores(self.tokenizer(query))
        ranked = sorted(range(len(scores)), key=lambda i: scores[i], reverse=True)[:top_k]
        return [
            {**self.documents[idx], "score": float(scores[idx])}
            for idx in ranked
        ]


class Reranker:
    """Cross-encoder re-ranking of candidate documents.

    Scores (query, document) pairs with CrossEncoder("BAAI/bge-reranker-base")
    and returns the top_k candidates in descending order.
    """

    def __init__(self, model: Optional[str] = None):
        reranker_config = CONFIG.get("models", {}).get("reranker", {})
        self.model_name = model or reranker_config.get("model_name", "BAAI/bge-reranker-base")

        logger.info("Loading reranker %s on %s", self.model_name, _device())
        self.model = CrossEncoder(self.model_name, device=_device())

    def rerank(
        self,
        query: str,
        candidates: List[Dict[str, Any]],
        top_k: int = 5,
    ) -> List[Dict[str, Any]]:
        """Re-score candidates against `query` and return the top_k."""
        if not candidates:
            return []
        pairs = [(query, doc["content"]) for doc in candidates]
        scores = self.model.predict(pairs)
        scored = sorted(zip(candidates, scores), key=lambda cs: float(cs[1]), reverse=True)[:top_k]
        return [{**doc, "score": float(score)} for doc, score in scored]