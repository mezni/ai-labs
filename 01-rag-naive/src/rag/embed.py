"""OpenRouter embedding generation."""

from __future__ import annotations

import logging
import time
from collections.abc import Sequence

from openai import OpenAI
from pydantic import BaseModel, ConfigDict, Field

from rag.config import Settings

logger = logging.getLogger(__name__)

DEFAULT_MAX_RETRIES = 3
DEFAULT_RETRY_DELAY = 1.0


class EmbeddingError(RuntimeError):
    """Raised when an embedding request fails or returns an invalid response."""


class Embedding(BaseModel):
    """A single text paired with its embedding vector."""

    model_config = ConfigDict(frozen=True)

    index: int = Field(ge=0)
    text: str
    vector: list[float]


class Embedder:
    """Generate embeddings through an OpenAI-compatible embedding service."""

    def __init__(
        self,
        client: OpenAI,
        *,
        model: str,
        max_retries: int = DEFAULT_MAX_RETRIES,
        retry_delay: float = DEFAULT_RETRY_DELAY,
    ) -> None:
        if not model:
            raise ValueError("model must be a non-empty embedding model id")
        if max_retries < 0:
            raise ValueError("max_retries must be >= 0")
        if retry_delay < 0:
            raise ValueError("retry_delay must be >= 0")
        self.client = client
        self.model = model
        self.max_retries = max_retries
        self.retry_delay = retry_delay

    def embed_texts(
        self, texts: Sequence[str], *, expected_dimension: int | None = None
    ) -> list[Embedding]:
        """Embed ``texts`` in a single request, retrying transient failures."""
        if not texts:
            return []

        ordered_texts = list(texts)
        response = self._request(ordered_texts)
        embeddings = self._parse(response, ordered_texts)

        dimensions = {len(embedding.vector) for embedding in embeddings}
        if len(dimensions) > 1:
            raise EmbeddingError(f"embedding response returned mixed dimensions: {dimensions}")

        if expected_dimension is not None:
            validate_dimension(embeddings, expected_dimension)
        return embeddings

    def embed_text(self, text: str, *, expected_dimension: int | None = None) -> Embedding:
        """Embed a single ``text``."""
        return self.embed_texts([text], expected_dimension=expected_dimension)[0]

    def _request(self, texts: list[str]):
        last_error: Exception | None = None
        for attempt in range(self.max_retries + 1):
            try:
                return self.client.embeddings.create(input=texts, model=self.model)
            except Exception as exc:
                last_error = exc
                logger.warning("Embedding request failed (attempt %s): %s", attempt + 1, exc)
                if attempt < self.max_retries:
                    time.sleep(self.retry_delay * (2**attempt))
        raise EmbeddingError(
            f"embedding request failed after {self.max_retries + 1} attempts: {last_error}"
        ) from last_error

    @staticmethod
    def _parse(response, texts: list[str]) -> list[Embedding]:
        data = getattr(response, "data", None)
        if not isinstance(data, list):
            raise EmbeddingError("embedding response did not contain a data list")
        if len(data) != len(texts):
            raise EmbeddingError(
                f"embedding response returned {len(data)} embeddings for {len(texts)} inputs"
            )
        embeddings: list[Embedding] = []
        for item in data:
            index = getattr(item, "index", None)
            vector = getattr(item, "embedding", None)
            if not isinstance(index, int) or not 0 <= index < len(texts):
                raise EmbeddingError(f"embedding response has invalid index: {index!r}")
            if not vector or not all(isinstance(value, float) for value in vector):
                raise EmbeddingError(f"embedding response has invalid vector at index {index}")
            embeddings.append(Embedding(index=index, text=texts[index], vector=vector))
        return embeddings


def validate_dimension(embeddings: Sequence[Embedding], expected_dimension: int) -> None:
    """Fail safely unless every embedding matches ``expected_dimension``."""
    if expected_dimension < 1:
        raise ValueError("expected_dimension must be >= 1")
    for embedding in embeddings:
        if len(embedding.vector) != expected_dimension:
            raise EmbeddingError(
                f"embedding dimension {len(embedding.vector)} does not match "
                f"expected dimension {expected_dimension} (index {embedding.index})"
            )


def build_embedder(settings: Settings) -> Embedder:
    """Create an :class:`Embedder` from the configured OpenRouter settings."""
    if not settings.openrouter_embedding_model:
        raise ValueError("OPENROUTER_EMBEDDING_MODEL is not configured")
    if not settings.openrouter_api_key:
        logger.warning("OPENROUTER_API_KEY is not set; embedding requests will fail")
    client = OpenAI(
        base_url=settings.openrouter_base_url,
        api_key=settings.openrouter_api_key or None,
    )
    return Embedder(client, model=settings.openrouter_embedding_model)
