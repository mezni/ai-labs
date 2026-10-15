from collections.abc import Iterator, Sequence
from typing import Protocol


class EmbeddingProvider(Protocol):
    name: str
    model: str
    dim: int

    def embed(self, texts: Sequence[str]) -> list[list[float]]: ...


def batched(items: Sequence[str], size: int) -> Iterator[Sequence[str]]:
    for start in range(0, len(items), size):
        yield items[start : start + size]


def embed_all(
    provider: EmbeddingProvider, texts: Sequence[str], batch_size: int
) -> list[list[float]]:
    """Embed texts in batches, keeping order and checking the output shape."""
    vectors: list[list[float]] = []
    for batch in batched(texts, batch_size):
        vectors.extend(provider.embed(batch))
    if len(vectors) != len(texts):
        raise ValueError(f"expected {len(texts)} vectors, got {len(vectors)}")
    for vector in vectors:
        if len(vector) != provider.dim:
            raise ValueError(f"expected {provider.dim} dimensions, got {len(vector)}")
    return vectors
