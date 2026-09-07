"""Tests for Embedder."""
import os
import sys
import types

import numpy as np
import pytest

from src.knowledge.embeddings import Embedder


@pytest.fixture(autouse=True)
def _fake_sentence_transformers(monkeypatch):
    """Fake the optional sentence-transformers import to keep tests offline."""

    class FakeModel:
        def encode(self, texts):
            return np.zeros((len(texts), 3))

    class FakeSentenceTransformer(FakeModel):
        def __init__(self, name: str) -> None:
            self.name = name

    module = types.ModuleType("sentence_transformers")
    module.SentenceTransformer = FakeSentenceTransformer
    monkeypatch.setitem(sys.modules, "sentence_transformers", module)
    yield FakeSentenceTransformer


def test_embed_documents_empty_returns_empty_list():
    embedder = Embedder(backend="local", model_name="unused")

    assert embedder.embed_documents([]) == []


def test_embed_text_returns_first_embedding(monkeypatch):
    embedder = Embedder(backend="local", model_name="unused")
    monkeypatch.setattr(embedder, "embed_documents", lambda texts: [[0.1], [0.2]])

    assert embedder.embed_text("hello") == [0.1]


def test_local_backend_embeds_with_local_model(_fake_sentence_transformers):
    embedder = Embedder(backend="local", model_name="all-MiniLM-L6-v2")

    assert embedder.backend == "local"
    assert isinstance(embedder._model, _fake_sentence_transformers)

    vectors = embedder.embed_documents(["hello", "world"])

    assert len(vectors) == 2
    assert all(len(vector) == 3 for vector in vectors)
    assert all(
        isinstance(value, float) for vector in vectors for value in vector
    )


def test_unknown_backend_raises_value_error():
    with pytest.raises(ValueError, match="Unknown embedder backend 'bogus'"):
        Embedder(backend="bogus")


def test_openrouter_requires_api_key(monkeypatch):
    monkeypatch.delenv("OPENROUTER_API_KEY", raising=False)

    with pytest.raises(RuntimeError, match="OPENROUTER_API_KEY is not set"):
        Embedder(backend="openrouter")


def test_local_backend_defaults_from_config():
    embedder = Embedder()

    assert embedder.backend == "local"