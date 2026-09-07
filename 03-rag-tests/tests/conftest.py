"""Shared fixtures for unit tests. No network or live API access."""

from __future__ import annotations

import copy
from typing import Any

import pytest

FIXTURE_CONFIG: dict[str, Any] = {
    "api": {"timeout": 60, "max_retries": 2},
    "models": {
        "chat": {
            "provider": "openrouter",
            "openrouter_url": "https://openrouter.ai/api/v1/chat/completions",
            "model": "minimax/MiniMax-M2",
            "max_tokens": 4096,
            "temperature": 0.3,
            "timeout_seconds": 120,
        },
        "embedding": {
            "provider": "openrouter",
            "openrouter_url": "https://openrouter.ai/api/v1/embeddings",
            "model": "openai/text-embedding-3-small",
            "batch_size": 64,
            "timeout_seconds": 120,
        },
        "reranker": {
            "provider": "openrouter",
            "openrouter_url": "https://openrouter.ai/api/v1/rerank",
            "model": "cohere/rerank-v3.5",
            "top_n": 5,
            "timeout_seconds": 120,
        },
    },
}


@pytest.fixture
def llm_config() -> dict[str, Any]:
    """Return a deep copy of the fixture LLM config."""
    return copy.deepcopy(FIXTURE_CONFIG)


@pytest.fixture
def env_openrouter(monkeypatch) -> None:
    """Provide an OPENROUTER_API_KEY in the environment for allowed calls."""
    monkeypatch.setenv("OPENROUTER_API_KEY", "test-openrouter-key")


@pytest.fixture
def patch_llm_config(monkeypatch, llm_config):
    """Monkeypatch LLMClient.load_config to return the fixture config."""
    import src.llm.llm_client as llm_client

    monkeypatch.setattr(llm_client, "load_config", lambda: llm_config)
    return llm_config


@pytest.fixture
def patch_knowledge_config(monkeypatch, llm_config):
    """Monkeypatch the knowledge package's load_config to the fixture config."""
    import src.knowledge.embeddings as embeddings

    monkeypatch.setattr(embeddings, "load_config", lambda: llm_config)
    return llm_config


class FakeEmbedder:
    """Deterministic embedder: projects text onto a small vector by topic."""

    @staticmethod
    def _vec(text: str) -> list[float]:
        low = text.lower()
        if "access" in low:
            return [0.0, 1.0]
        if "backup" in low:
            return [1.0, 0.0]
        if "leave" in low:
            return [0.5, 0.5]
        return [0.3, 0.3]

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        return [self._vec(t) for t in texts]

    def embed_text(self, text: str) -> list[float]:
        return self._vec(text)

    def close(self) -> None:
        pass


@pytest.fixture
def fake_embedder() -> FakeEmbedder:
    return FakeEmbedder()