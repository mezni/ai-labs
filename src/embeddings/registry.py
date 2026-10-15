from config import ConfigError, Settings
from embeddings.base import EmbeddingProvider
from embeddings.hashing import HashingEmbedder
from embeddings.openai_provider import OpenAIEmbedder


def get_provider(settings: Settings) -> EmbeddingProvider:
    if settings.embedding_provider == "hashing":
        return HashingEmbedder(dim=settings.embedding_dim)
    if settings.embedding_provider == "openai":
        if not settings.openai_api_key:
            raise ConfigError("OPENAI_API_KEY is required when EMBEDDING_PROVIDER=openai")
        return OpenAIEmbedder(
            api_key=settings.openai_api_key,
            model=settings.embedding_model,
            dim=settings.embedding_dim,
        )
    raise ConfigError(f"Unknown EMBEDDING_PROVIDER: {settings.embedding_provider!r}")
