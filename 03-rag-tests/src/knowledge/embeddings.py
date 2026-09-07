"""Embeddings, BM25 index, and reranker components for retrieval.

All three model families are configured from config/llm_config.yaml
(models.embedding and models.reranker). API-key protected endpoints use
OPENROUTER_API_KEY from the environment (.env).
"""

from __future__ import annotations

import os
import re
from typing import Callable

import httpx
from dotenv import load_dotenv
from rank_bm25 import BM25Okapi

from src.utils import get_logger, load_config

load_dotenv()

logger = get_logger(__name__)


def _require_api_key() -> str:
    api_key = os.getenv("OPENROUTER_API_KEY")
    if not api_key:
        raise RuntimeError("OPENROUTER_API_KEY is not set")
    return api_key


def _auth_headers() -> dict[str, str]:
    return {
        "Authorization": f"Bearer {_require_api_key()}",
        "Content-Type": "application/json",
    }


class EmbeddingGenerator:
    """Embed texts into vectors using the OpenRouter embeddings API."""

    def __init__(
        self,
        transport: httpx.BaseTransport | None = None,
    ) -> None:
        cfg = load_config().get("models", {}).get("embedding", {})
        self.model = cfg.get("model", "openai/text-embedding-3-small")
        self.url = cfg.get(
            "openrouter_url", "https://openrouter.ai/api/v1/embeddings"
        )
        self.batch_size = cfg.get("batch_size", 64)
        timeout = cfg.get("timeout_seconds", 120)

        self._client = httpx.Client(
            transport=transport,
            timeout=timeout,
            headers=_auth_headers(),
        )
        self._dimensions: int | None = None
        logger.debug(
            "EmbeddingGenerator ready (model=%s, batch_size=%d)",
            self.model,
            self.batch_size,
        )

    @property
    def dimensions(self) -> int | None:
        """Vector dimension, available after the first embed call."""
        return self._dimensions

    def embed_text(self, text: str) -> list[float]:
        """Embed a single text into a vector."""
        return self.embed_documents([text])[0]

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed texts into vectors, batching requests as needed."""
        if not texts:
            return []

        vectors: list[list[float]] = []
        for start in range(0, len(texts), self.batch_size):
            batch = texts[start : start + self.batch_size]
            logger.debug(
                "Embedding batch %d-%d of %d",
                start,
                start + len(batch),
                len(texts),
            )
            response = self._client.post(
                self.url,
                json={"model": self.model, "input": batch},
            )
            response.raise_for_status()
            vectors.extend(item["embedding"] for item in response.json()["data"])

        self._dimensions = len(vectors[0])
        logger.debug("Embedded %d texts (dim=%d)", len(texts), self._dimensions)
        return vectors

    def close(self) -> None:
        self._client.close()

    def __enter__(self) -> "EmbeddingGenerator":
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()


class BM25Index:
    """In-memory BM25 sparse index built from chunk texts."""

    def __init__(self, tokenizer: Callable[[str], list[str]] | None = None) -> None:
        self._bm25: BM25Okapi | None = None
        self._documents: list[str] = []
        self._tokenizer = tokenizer or self.tokenize

    @staticmethod
    def tokenize(text: str) -> list[str]:
        """Lowercase, drop punctuation, and split on whitespace."""
        return re.findall(r"[a-z0-9]+", text.lower())

    def add_documents(self, documents: list[str]) -> "BM25Index":
        """Build (or rebuild) the index over the given documents."""
        self._documents = list(documents)
        self._bm25 = BM25Okapi([self._tokenizer(doc) for doc in self._documents])
        logger.debug("BM25 index rebuilt over %d documents", len(self._documents))
        return self

    def get_scores(self, query: str) -> list[float]:
        """BM25 scores of every document for the query."""
        if self._bm25 is None:
            raise RuntimeError(
                "BM25 index is empty; call add_documents() before searching"
            )
        return self._bm25.get_scores(self._tokenizer(query))

    def search(self, query: str, top_k: int = 5) -> list[dict[str, object]]:
        """Return top_k documents as {index, score, text} sorted by relevance."""
        scores = self.get_scores(query)
        ranked = sorted(
            range(len(scores)),
            key=lambda i: scores[i],
            reverse=True,
        )
        results = [
            {
                "index": idx,
                "score": float(scores[idx]),
                "text": self._documents[idx],
            }
            for idx in ranked[:top_k]
        ]
        return [r for r in results if r["score"] > 0]

    def has_documents(self) -> bool:
        return self._bm25 is not None

    def __len__(self) -> int:
        return len(self._documents)


class Reranker:
    """Re-score candidate documents against a query via OpenRouter's rerank API."""

    def __init__(
        self,
        transport: httpx.BaseTransport | None = None,
    ) -> None:
        cfg = load_config().get("models", {}).get("reranker", {})
        self.model = cfg.get("model", "cohere/rerank-v3.5")
        self.url = cfg.get("openrouter_url", "https://openrouter.ai/api/v1/rerank")
        self.top_n = cfg.get("top_n", 5)
        timeout = cfg.get("timeout_seconds", 120)

        self._client = httpx.Client(
            transport=transport,
            timeout=timeout,
            headers=_auth_headers(),
        )
        logger.debug("Reranker ready (model=%s, top_n=%d)", self.model, self.top_n)

    def rerank(
        self,
        query: str,
        documents: list[str],
        top_n: int | None = None,
    ) -> list[str]:
        """Rerank documents for a query; return them in relevance order."""
        return [text for text, _ in self.rerank_with_scores(query, documents, top_n)]

    def rerank_with_scores(
        self,
        query: str,
        documents: list[str],
        top_n: int | None = None,
    ) -> list[tuple[str, float]]:
        """Rerank documents; return (text, relevance_score) pairs, sorted by score."""
        if not documents:
            return []
        top_n = top_n or self.top_n
        response = self._client.post(
            self.url,
            json={
                "model": self.model,
                "query": query,
                "documents": documents,
                "top_n": top_n,
            },
        )
        response.raise_for_status()
        results = sorted(
            response.json()["results"],
            key=lambda r: r["relevance_score"],
            reverse=True,
        )
        return [(documents[r["index"]], float(r["relevance_score"])) for r in results]

    def close(self) -> None:
        self._client.close()

    def __enter__(self) -> "Reranker":
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()