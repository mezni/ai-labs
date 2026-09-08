"""Tests for ``rag.config`` settings."""

from pathlib import Path

import pytest
from pydantic import ValidationError

from rag.config import PROJECT_ROOT, Settings


def test_defaults_loaded_without_environment():
    settings = Settings()
    assert settings.openrouter_api_key == ""
    assert settings.openrouter_base_url == "https://openrouter.ai/api/v1"
    assert settings.openrouter_embedding_model == ""
    assert settings.openrouter_model == ""
    assert settings.chunk_size == 800
    assert settings.chunk_overlap == 150
    assert settings.top_k == 5


def test_paths_anchored_to_project_root():
    settings = Settings()
    assert settings.data_dir == PROJECT_ROOT / "data" / "raw"
    assert settings.index_path == PROJECT_ROOT / "indexes" / "faiss.index"
    assert settings.metadata_path == PROJECT_ROOT / "indexes" / "metadata.json"
    assert settings.document_state_path == PROJECT_ROOT / "indexes" / "document_state.json"
    assert settings.manifest_path == PROJECT_ROOT / "indexes" / "index_manifest.json"
    assert settings.log_path == PROJECT_ROOT / "logs" / "indexing.log"


def test_environment_overrides_defaults(monkeypatch):
    monkeypatch.setenv("OPENROUTER_BASE_URL", "http://localhost:9000/v1")
    monkeypatch.setenv("OPENROUTER_EMBEDDING_MODEL", "test-embed")
    monkeypatch.setenv("OPENROUTER_MODEL", "test-gen")
    monkeypatch.setenv("CHUNK_SIZE", "400")

    settings = Settings()
    assert settings.openrouter_base_url == "http://localhost:9000/v1"
    assert settings.openrouter_embedding_model == "test-embed"
    assert settings.openrouter_model == "test-gen"
    assert settings.chunk_size == 400


def test_paths_overridable_via_environment(monkeypatch):
    monkeypatch.setenv("DATA_DIR", "custom_data")
    assert Settings().data_dir == Path("custom_data")


@pytest.mark.parametrize(
    "overrides",
    [
        {"chunk_size": 0},
        {"chunk_overlap": -1},
        {"top_k": 0},
    ],
)
def test_invalid_values_rejected(overrides):
    with pytest.raises(ValidationError):
        Settings().model_validate(overrides)
