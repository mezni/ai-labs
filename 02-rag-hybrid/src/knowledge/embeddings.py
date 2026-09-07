"""OpenRouter-backed text embedding generator."""

from __future__ import annotations

import os

import httpx
from dotenv import load_dotenv

from src.utils.config_loader import load_all_configs
from src.utils.logger import get_logger

load_dotenv()

logger = get_logger(__name__)


class EmbeddingGenerator:
    """Embed text into vectors using the OpenRouter embeddings API."""

    def __init__(
        self,
        model_name: str | None = None,
        url: str | None = None,
        batch_size: int | None = None,
        timeout_seconds: int | None = None,
    ) -> None:
        configs = load_all_configs()
        self._cfg = configs.get("llm", {}).get("embeddings", {})

        self.model = model_name or self._cfg.get(
            "model", "openai/text-embedding-3-small"
        )
        self.url = url or self._cfg.get(
            "openrouter_url", "https://openrouter.ai/api/v1/embeddings"
        )
        self.batch_size = batch_size or self._cfg.get("batch_size", 64)

        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")

        effective_timeout = timeout_seconds or self._cfg.get("timeout_seconds", 120)
        self._client = httpx.Client(
            timeout=effective_timeout,
            headers={"Authorization": f"Bearer {api_key}"},
        )
        logger.debug(
            "EmbeddingGenerator ready (model=%s, url=%s, batch_size=%d)",
            self.model,
            self.url,
            self.batch_size,
        )

    @property
    def dimensions(self) -> int | None:
        """Vector dimension, available after the first embed call."""
        return getattr(self, "_dimensions", None)

    def embed_text(self, text: str) -> list[float]:
        """Embed a single text into a vector."""
        return self.embed_documents([text])[0]

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed a list of texts into vectors, batching requests as needed."""
        if not texts:
            return []

        vectors: list[list[float]] = []
        for i in range(0, len(texts), self.batch_size):
            batch = texts[i : i + self.batch_size]
            logger.debug("Embedding batch %d-%d of %d", i, i + len(batch), len(texts))
            response = self._client.post(
                self.url,
                json={"model": self.model, "input": batch},
            )
            response.raise_for_status()
            data = response.json()["data"]
            vectors.extend(item["embedding"] for item in data)

        self._dimensions = len(vectors[0])
        logger.debug("Embedded %d texts (dim=%d)", len(texts), self._dimensions)
        return vectors

    def close(self) -> None:
        """Close the underlying HTTP client."""
        self._client.close()

    def __enter__(self) -> "EmbeddingGenerator":
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()