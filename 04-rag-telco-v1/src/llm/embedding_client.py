"""Embedding client."""

import os
from typing import Any

import httpx
from dotenv import load_dotenv

from src.utils import get_logger, load_config

load_dotenv()

logger = get_logger(__name__)


class EmbeddingClient:
    """Generate text embeddings using the configured embedding model."""

    def __init__(
        self,
        model: str | None = None,
        transport: httpx.BaseTransport | None = None,
    ) -> None:
        config = load_config()

        embedding_config = config.get("models", {}).get("embedding", {})
        api_config = config.get("api", {})

        self.model = model or embedding_config.get(
            "model",
            "openai/text-embedding-3-small",
        )

        self.dimensions = embedding_config.get("dimensions", 1536)

        self.url = embedding_config.get(
            "openrouter_url",
            "https://openrouter.ai/api/v1/embeddings",
        )

        timeout = embedding_config.get(
            "timeout_seconds",
            api_config.get("timeout", 120),
        )

        api_key = os.getenv("OPENROUTER_API_KEY")

        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")

        self._client = httpx.Client(
            transport=transport,
            timeout=timeout,
            headers={
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
            },
        )

        logger.debug(
            "EmbeddingClient ready (model=%s, dimensions=%d)",
            self.model,
            self.dimensions,
        )

    def embed(self, text: str) -> list[float]:
        """Generate an embedding for one piece of text."""

        if not text.strip():
            raise ValueError("Cannot embed empty text")

        response = self._client.post(
            self.url,
            json={
                "model": self.model,
                "input": text,
            },
        )

        response.raise_for_status()

        data: dict[str, Any] = response.json()

        embedding = data["data"][0]["embedding"]

        return embedding

    def embed_many(self, texts: list[str]) -> list[list[float]]:
        """Generate embeddings for multiple texts."""

        if not texts:
            return []

        response = self._client.post(
            self.url,
            json={
                "model": self.model,
                "input": texts,
            },
        )

        response.raise_for_status()

        data: dict[str, Any] = response.json()

        embeddings = [
            item["embedding"]
            for item in sorted(
                data["data"],
                key=lambda item: item["index"],
            )
        ]

        return embeddings

    def close(self) -> None:
        """Close the HTTP client."""

        self._client.close()