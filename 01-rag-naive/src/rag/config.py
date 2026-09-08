"""Centralized configuration for the RAG pipeline."""

from pathlib import Path

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict

PROJECT_ROOT = Path(__file__).resolve().parents[2]


class Settings(BaseSettings):
    """Application settings loaded from environment variables and ``.env``."""

    model_config = SettingsConfigDict(
        env_file=PROJECT_ROOT / ".env",
        env_file_encoding="utf-8",
        case_sensitive=False,
    )

    openrouter_api_key: str = ""
    openrouter_base_url: str = "https://openrouter.ai/api/v1"
    openrouter_embedding_model: str = ""
    openrouter_model: str = ""

    chunk_size: int = Field(default=800, ge=1)
    chunk_overlap: int = Field(default=150, ge=0)
    top_k: int = Field(default=5, ge=1)

    data_dir: Path = PROJECT_ROOT / "data" / "raw"
    index_path: Path = PROJECT_ROOT / "indexes" / "faiss.index"
    metadata_path: Path = PROJECT_ROOT / "indexes" / "metadata.json"
    document_state_path: Path = PROJECT_ROOT / "indexes" / "document_state.json"
    manifest_path: Path = PROJECT_ROOT / "indexes" / "index_manifest.json"
    log_path: Path = PROJECT_ROOT / "logs" / "indexing.log"


settings = Settings()
