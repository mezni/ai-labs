from __future__ import annotations

from pathlib import Path

from pydantic import field_validator
from pydantic_settings import BaseSettings, SettingsConfigDict

PROJECT_ROOT = Path(__file__).resolve().parent.parent


class Settings(BaseSettings):
    """Centralized configuration for the RAG DataOps pipeline.

    Secrets and overrides come from the environment / `.env`; every path is
    anchored to the project root so the pipeline works from any CWD.
    """

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
    )

    openrouter_api_key: str = ""
    openrouter_base_url: str = "https://openrouter.ai/api/v1"
    openrouter_embedding_model: str = "openai/text-embedding-3-small"
    openrouter_model: str = "minimax/minimax-m3"

    chroma_dir: Path = PROJECT_ROOT / "indexes" / "chroma"
    index_registry: Path = PROJECT_ROOT / "indexes" / "index_registry.json"
    document_catalog: Path = PROJECT_ROOT / "indexes" / "document_catalog.json"

    data_dir: Path = PROJECT_ROOT / "data" / "raw"
    log_path: Path = PROJECT_ROOT / "logs" / "rag-dataops.log"

    chunk_size: int = 800
    chunk_overlap: int = 150
    top_k: int = 5

    @field_validator("chunk_size", "chunk_overlap", "top_k")
    @classmethod
    def _positive_int(cls, value: int) -> int:
        if value <= 0:
            raise ValueError(f"must be a positive integer, got {value}")
        return value

    @field_validator("chroma_dir", "index_registry", "document_catalog", "data_dir", "log_path")
    @classmethod
    def _anchor_paths(cls, value: Path) -> Path:
        if not value.is_absolute():
            value = PROJECT_ROOT / value
        return value

    @property
    def chroma_versions_dir(self) -> Path:
        return self.chroma_dir / "versions"

    def ensure_dirs(self) -> None:
        """Create every directory the pipeline needs to write to."""
        self.data_dir.mkdir(parents=True, exist_ok=True)
        self.chroma_versions_dir.mkdir(parents=True, exist_ok=True)
        self.log_path.parent.mkdir(parents=True, exist_ok=True)


def get_settings() -> Settings:
    return Settings()