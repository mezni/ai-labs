import math
from collections.abc import Sequence

import pytest

from config import ConfigError, Settings
from embeddings.base import embed_all
from embeddings.hashing import HashingEmbedder
from embeddings.registry import get_provider


def dot(a: list[float], b: list[float]) -> float:
    return sum(x * y for x, y in zip(a, b, strict=True))


class RecordingProvider:
    name = "recording"
    model = "recording-v1"
    dim = 3

    def __init__(self) -> None:
        self.batches: list[list[str]] = []

    def embed(self, texts: Sequence[str]) -> list[list[float]]:
        self.batches.append(list(texts))
        return [[float(len(text)), 0.0, 0.0] for text in texts]


class WrongDimensionProvider:
    name = "wrong"
    model = "wrong-v1"
    dim = 3

    def embed(self, texts: Sequence[str]) -> list[list[float]]:
        return [[0.0, 0.0] for _ in texts]


def test_hashing_is_deterministic_and_normalized() -> None:
    embedder = HashingEmbedder(dim=64)
    first = embedder.embed(["Zone B roaming fee"])[0]
    second = embedder.embed(["Zone B roaming fee"])[0]
    assert first == second
    assert len(first) == 64
    assert math.isclose(dot(first, first), 1.0)


def test_similar_texts_are_closer() -> None:
    embedder = HashingEmbedder()
    query, similar, different = embedder.embed(
        ["roaming data fee in zone b", "zone b roaming fee", "esim activation qr code error"]
    )
    assert dot(query, similar) > dot(query, different)


def test_embed_all_batches_and_keeps_order() -> None:
    provider = RecordingProvider()
    vectors = embed_all(provider, ["a", "bb", "ccc", "dddd", "eeeee"], batch_size=2)
    assert [len(batch) for batch in provider.batches] == [2, 2, 1]
    assert [vector[0] for vector in vectors] == [1.0, 2.0, 3.0, 4.0, 5.0]


def test_embed_all_rejects_wrong_dimension() -> None:
    with pytest.raises(ValueError, match="dimensions"):
        embed_all(WrongDimensionProvider(), ["a"], batch_size=10)


def test_get_provider_hashing() -> None:
    settings = Settings(_env_file=None, embedding_provider="hashing", embedding_dim=32)
    provider = get_provider(settings)
    assert isinstance(provider, HashingEmbedder)
    assert provider.dim == 32


def test_unknown_provider_raises() -> None:
    with pytest.raises(ConfigError):
        get_provider(Settings(_env_file=None, embedding_provider="nope"))


def test_openai_provider_requires_api_key() -> None:
    settings = Settings(_env_file=None, embedding_provider="openai", openai_api_key=None)
    with pytest.raises(ConfigError, match="OPENAI_API_KEY"):
        get_provider(settings)
