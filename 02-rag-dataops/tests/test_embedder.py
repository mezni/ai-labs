from __future__ import annotations

import pytest
from types import SimpleNamespace

from src.embeddings.embedder import (
    DimensionMismatchError,
    Embedder,
    EmbeddingError,
    embedding_model_changed,
)
from tests.conftest import FakeEmbeddingsClient, text_vector


def test_dimension_captured_on_first_embedding(fake_embedder):
    assert fake_embedder.dimension is None
    vectors = fake_embedder.embed(["a", "b"])
    assert fake_embedder.dimension == 4
    assert len(vectors) == 2
    assert len(vectors[0]) == 4


def test_embeddings_are_deterministic_for_same_text(fake_embedder):
    assert fake_embedder.embed(["abc"]) == fake_embedder.embed(["abc"])


class _Embeddings:
    def __init__(self, owner):
        self.owner = owner

    def create(self, model, input):
        return self.owner.create(model, input)


class _SwitchingClient:
    def __init__(self):
        self.embeddings = _Embeddings(self)
        self.create_calls = {"first": False}

    def create(self, model, input):
        items = list(input)
        if not self.create_calls["first"]:
            self.create_calls["first"] = True
            dims = [4, 4]
        else:
            dims = [8, 8]
        return SimpleNamespace(
            data=[
                SimpleNamespace(embedding=text_vector(t, dims[i]), index=i)
                for i, t in enumerate(items)
            ]
        )


def test_dimension_mismatch_is_rejected():
    embedder = Embedder(
        api_key="sk-test",
        base_url="https://x",
        model="m",
        client=_SwitchingClient(),
    )
    embedder.embed(["first"])
    with pytest.raises(DimensionMismatchError):
        embedder.embed(["second", "third"])


def test_missing_key_raises_embedding_error():
    embedder = Embedder(api_key="", base_url="https://x", model="m")
    assert not embedder.configured
    with pytest.raises(EmbeddingError):
        embedder.embed(["x"])


def test_request_failure_raises_embedding_error():
    class BrokenClient:
        def __init__(self):
            self.embeddings = _Embeddings(self)

        def create(self, model, input):
            raise RuntimeError("network down")

    embedder = Embedder(api_key="k", base_url="https://x", model="m", client=BrokenClient())
    with pytest.raises(EmbeddingError):
        embedder.embed(["x"])


def test_batching_groups_by_batch_size():
    client = FakeEmbeddingsClient()
    embedder = Embedder(
        api_key="k", base_url="https://x", model="m", client=client, embed_batch_size=2
    )
    embedder.embed([f"t{i}" for i in range(5)])
    assert [len(batch) for batch in client.calls] == [2, 2, 1]


def test_model_change_detected_only_when_previous_differs():
    assert not embedding_model_changed(None, "m2")
    assert not embedding_model_changed("m1", "m1")
    assert embedding_model_changed("m1", "m2")


def test_model_and_base_url_registered_on_embedder(fake_embedder):
    assert fake_embedder.model == "test-embed"
    assert fake_embedder.base_url == "https://test-gateway.example/v1"