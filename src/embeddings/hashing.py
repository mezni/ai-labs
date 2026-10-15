import hashlib
import math
import re
from collections.abc import Sequence

_WORD = re.compile(r"[a-z0-9$]+")


class HashingEmbedder:
    """Offline, deterministic bag-of-words embedder. For tests and demos, not for quality."""

    name = "hashing"
    model = "hashing-v1"

    def __init__(self, dim: int = 1536) -> None:
        self.dim = dim

    def embed(self, texts: Sequence[str]) -> list[list[float]]:
        return [self._embed_one(text) for text in texts]

    def _embed_one(self, text: str) -> list[float]:
        vector = [0.0] * self.dim
        for word in _WORD.findall(text.lower()):
            digest = hashlib.sha256(word.encode()).digest()
            index = int.from_bytes(digest[:4], "big") % self.dim
            vector[index] += 1.0 if digest[4] % 2 == 0 else -1.0
        norm = math.sqrt(sum(value * value for value in vector))
        if norm == 0.0:
            vector[0] = 1.0  # empty text still gets a valid unit vector
            return vector
        return [value / norm for value in vector]
