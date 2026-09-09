from __future__ import annotations

import hashlib
from types import SimpleNamespace

import pytest

from src.config import Settings
from src.embeddings.embedder import Embedder

EMBED_DIM = 4


def text_vector(text: str, dimension: int = EMBED_DIM) -> list[float]:
    """Deterministic pseudo-embedding derived from text (same text → same vector)."""
    digest = hashlib.sha256(text.encode("utf-8")).hexdigest()
    return [int(digest[i : i + 2], 16) / 255 for i in range(0, dimension * 2, 2)]


class _FakeEmbeddings:
    """OpenAI-v3 namespace: ``client.embeddings.create(model, input)``."""

    def __init__(self, owner: FakeEmbeddingsClient):
        self.owner = owner

    def create(self, model: str, input) -> SimpleNamespace:
        return self.owner.create(model, input)


class FakeEmbeddingsClient:
    """OpenAI-compatible fake returning deterministic ``Embedding`` objects."""

    def __init__(self, dimension: int = EMBED_DIM):
        self.dimension = dimension
        self.calls: list[list[str]] = []
        self.embeddings = _FakeEmbeddings(self)

    def create(self, model: str, input) -> SimpleNamespace:
        items = list(input) if isinstance(input, (list, tuple)) else [input]
        self.calls.append(items)
        data = [
            SimpleNamespace(embedding=text_vector(text), index=i)
            for i, text in enumerate(items)
        ]
        return SimpleNamespace(model=model, data=data)


@pytest.fixture
def settings(tmp_path):
    """Isolated Settings rooted in a temp directory (no `.env` interference)."""
    return Settings(
        _env_file=None,
        openrouter_api_key="",
        openrouter_base_url="https://test-gateway.example/v1",
        openrouter_embedding_model="test-embed",
        chroma_dir=tmp_path / "indexes" / "chroma",
        index_registry=tmp_path / "indexes" / "index_registry.json",
        document_catalog=tmp_path / "indexes" / "document_catalog.json",
        data_dir=tmp_path / "data" / "raw",
        log_path=tmp_path / "logs" / "rag-dataops.log",
    )


@pytest.fixture
def fake_embedder():
    return Embedder(
        api_key="sk-test",
        base_url="https://test-gateway.example/v1",
        model="test-embed",
        client=FakeEmbeddingsClient(dimension=EMBED_DIM),
    )


@pytest.fixture
def fake_embedding_client() -> FakeEmbeddingsClient:
    return FakeEmbeddingsClient(dimension=EMBED_DIM)