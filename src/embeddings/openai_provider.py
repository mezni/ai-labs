from collections.abc import Sequence

from openai import OpenAI


class OpenAIEmbedder:
    name = "openai"

    def __init__(self, *, api_key: str, model: str, dim: int) -> None:
        self.model = model
        self.dim = dim
        self._client = OpenAI(api_key=api_key)

    def embed(self, texts: Sequence[str]) -> list[list[float]]:
        # `dimensions` is supported by the text-embedding-3 family only.
        response = self._client.embeddings.create(
            model=self.model, input=list(texts), dimensions=self.dim
        )
        return [item.embedding for item in sorted(response.data, key=lambda d: d.index)]
