"""Embedding client for generating text embeddings."""

import os

import httpx
from dotenv import load_dotenv

from src.utils import get_logger, load_config

load_dotenv()

logger = get_logger(__name__)

DEFAULT_EMBEDDING_URL = "https://api.openai.com/v1/embeddings"


class EmbeddingClient:
    """Generate text embeddings via an OpenAI-compatible embeddings API.

    Settings (model, dimensions, timeout, url) come from the ``models.embedding``
    section of config/llm_config.yaml. An API key is only required for live
    calls; tests can inject an ``httpx`` MockTransport and skip authentication.
    """

    def __init__(
        self,
        model: str | None = None,
        api_key: str | None = None,
        transport: httpx.BaseTransport | None = None,
    ):
        embedding_config = load_config().get("models", {}).get("embedding", {})

        self.model = model or embedding_config.get(
            "model", "text-embedding-3-small"
        )
        self.dimensions = embedding_config.get("dimensions", 1536)
        self.url = embedding_config.get("url", DEFAULT_EMBEDDING_URL)
        timeout = embedding_config.get("timeout_seconds", 120)

        api_key = api_key or os.getenv("OPENAI_API_KEY")

        headers: dict[str, str] = {"Content-Type": "application/json"}
        if api_key:
            headers["Authorization"] = f"Bearer {api_key}"
        else:
            logger.warning("No OPENAI_API_KEY set; embedding requests will be unauthenticated")

        self._client = httpx.Client(transport=transport, timeout=timeout, headers=headers)
        logger.debug(
            "EmbeddingClient ready (model=%s, dimensions=%d)",
            self.model,
            self.dimensions,
        )

    def embed(self, text: str) -> list[float]:
        """Return the embedding vector for a single text input."""
        return self._embed_request(text)[0]

    def embed_many(self, texts: list[str]) -> list[list[float]]:
        """Return embedding vectors for a batch of texts, in input order."""
        return self._embed_request(texts)

    def _embed_request(self, payload: str | list[str]) -> list[list[float]]:
        """Post a single string or a list of strings to the embeddings API."""
        logger.debug("Embedding %s with %s", type(payload).__name__, self.model)
        response = self._client.post(
            self.url,
            json={"model": self.model, "input": payload},
        )
        response.raise_for_status()
        data = response.json()
        ordered = sorted(data["data"], key=lambda item: item["index"])
        return [item["embedding"] for item in ordered]

    def close(self) -> None:
        self._client.close()