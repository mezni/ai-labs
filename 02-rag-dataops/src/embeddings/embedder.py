from __future__ import annotations

import logging
from collections.abc import Iterable, Iterator
from typing import Any

from openai import OpenAI

logger = logging.getLogger("rag.dataops.embedding")


class EmbeddingError(Exception):
    """Embedding request or client configuration failure."""


class DimensionMismatchError(EmbeddingError):
    """A vector came back with a different dimension than the registered one."""


def _batched(items: Iterable[str], size: int) -> Iterator[list[str]]:
    batch: list[str] = []
    for item in items:
        batch.append(item)
        if len(batch) == size:
            yield batch
            batch = []
    if batch:
        yield batch


class Embedder:
    """OpenRouter OpenAI-compatible embedding client (PROJECT.md §10).

    Records the embedding model id and the vector dimension captured on the
    first embedding, and refuses to accept vectors that disagree with the
    registered dimension.
    """

    def __init__(
        self,
        api_key: str,
        base_url: str,
        model: str,
        dimensions: int | None = None,
        embed_batch_size: int = 64,
        client: OpenAI | None = None,
    ):
        self.model = model
        self.base_url = base_url
        self.embed_batch_size = embed_batch_size
        self._dimension = dimensions
        self._client = client or (
            OpenAI(api_key=api_key, base_url=base_url) if api_key else None
        )

    @property
    def dimension(self) -> int | None:
        return self._dimension

    @property
    def configured(self) -> bool:
        return self._client is not None

    def embed(self, texts: Iterable[str]) -> list[list[float]]:
        """Embed texts, capture the dimension on the first call, validate later."""
        if self._client is None:
            raise EmbeddingError("OPENROUTER_API_KEY is not set; cannot embed")
        vectors: list[list[float]] = []
        for batch in _batched(list(texts), self.embed_batch_size):
            response = self._request(batch)
            for _index, item in enumerate(response.data):
                vector = item.embedding
                if self._dimension is None:
                    self._dimension = len(vector)
                    logger.info(
                        "registration: embedding_model=%s dimension=%s",
                        self.model,
                        self._dimension,
                    )
                elif len(vector) != self._dimension:
                    raise DimensionMismatchError(
                        f"Dimension mismatch: expected {self._dimension} got "
                        f"{len(vector)} (model={self.model})"
                    )
                vectors.append(vector)
        return vectors

    def _request(self, batch: list[str]) -> Any:
        try:
            return self._client.embeddings.create(model=self.model, input=batch)
        except Exception as exc:
            logger.error("Embedding request failed: %s", exc)
            raise EmbeddingError(f"Embedding request failed: {exc}") from exc


def embedding_model_changed(previous: str | None, current: str) -> bool:
    """Model changes mean existing snapshots are incompatible → full reindex."""
    return previous is not None and previous != current