"""Embedding providers: local sentence-transformers and OpenRouter API."""
from __future__ import annotations

import os

import httpx
from dotenv import load_dotenv

from src.utils.config_loader import load_all_configs

load_dotenv()


class Embedder:
    """Embed text into vectors, backed by either a local model or an API provider.

    Pass `backend="local"` (default) to embed with a local sentence-transformers
    model, or `backend="openrouter"` to call the configured OpenRouter embeddings
    endpoint. If `backend` is omitted, it's read from config
    (`llm.embeddings.backend`, default "local").
    """

    def __init__(
        self,
        backend: str | None = None,
        model_name: str = "all-MiniLM-L6-v2",
    ) -> None:
        configs = load_all_configs()
        self._cfg = configs.get("llm", {}).get("embeddings", {})
        self.backend = backend or self._cfg.get("backend", "local")

        if self.backend == "local":
            self._init_local(model_name)
        elif self.backend == "openrouter":
            self._init_openrouter()
        else:
            raise ValueError(
                f"Unknown embedder backend {self.backend!r}; "
                "expected 'local' or 'openrouter'"
            )

    def _init_local(self, model_name: str) -> None:
        from sentence_transformers import SentenceTransformer

        self._model = SentenceTransformer(self._cfg.get("model_name", model_name))

    def _init_openrouter(self) -> None:
        self.model = self._cfg["model"]
        self.url = self._cfg.get(
            "openrouter_url", "https://openrouter.ai/api/v1/embeddings"
        )
        self.batch_size = self._cfg.get("batch_size", 64)
        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")
        self._client = httpx.Client(
            timeout=self._cfg.get("timeout_seconds", 120),
            headers={"Authorization": f"Bearer {api_key}"},
        )

    def embed_text(self, text: str) -> list[float]:
        """Embed a single text into a vector."""
        return self.embed_documents([text])[0]

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed a list of texts into vectors."""
        if not texts:
            return []
        if self.backend == "local":
            return self._embed_documents_local(texts)
        return self._embed_documents_openrouter(texts)

    def _embed_documents_local(self, texts: list[str]) -> list[list[float]]:
        return [vec.tolist() for vec in self._model.encode(texts)]

    def _embed_documents_openrouter(self, texts: list[str]) -> list[list[float]]:
        vectors: list[list[float]] = []
        for i in range(0, len(texts), self.batch_size):
            batch = texts[i : i + self.batch_size]
            response = self._client.post(
                self.url,
                json={"model": self.model, "input": batch},
            )
            response.raise_for_status()
            data = response.json()["data"]
            vectors.extend(item["embedding"] for item in data)
        return vectors