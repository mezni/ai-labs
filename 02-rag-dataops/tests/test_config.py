from __future__ import annotations

import pytest

from src.config import PROJECT_ROOT, Settings


def test_defaults_are_project_anchored():
    settings = Settings(_env_file=None)
    assert settings.chunk_size == 800
    assert settings.chunk_overlap == 150
    assert settings.top_k == 5
    assert settings.openrouter_base_url.startswith("https://")
    assert settings.openrouter_embedding_model
    assert settings.chroma_dir.is_absolute()
    assert settings.index_registry.is_absolute()
    assert settings.document_catalog.is_absolute()
    assert settings.data_dir.is_absolute()
    assert settings.log_path.is_absolute()
    assert settings.chroma_versions_dir == settings.chroma_dir / "versions"


def test_env_overrides(monkeypatch):
    monkeypatch.setenv("OPENROUTER_EMBEDDING_MODEL", "custom/model")
    monkeypatch.setenv("CHUNK_SIZE", "400")
    monkeypatch.setenv("TOP_K", "3")
    settings = Settings(_env_file=None)
    assert settings.openrouter_embedding_model == "custom/model"
    assert settings.chunk_size == 400
    assert settings.top_k == 3


def test_relative_paths_anchor_to_project_root():
    settings = Settings(_env_file=None, chroma_dir="somewhere/else")
    assert settings.chroma_dir == PROJECT_ROOT / "somewhere" / "else"


def test_invalid_positive_ints_rejected():
    with pytest.raises(ValueError):
        Settings(_env_file=None, chunk_size=0)
    with pytest.raises(ValueError):
        Settings(_env_file=None, top_k=-1)


def test_ensure_dirs_creates_required_directories(settings):
    settings.ensure_dirs()
    assert settings.data_dir.is_dir()
    assert settings.chroma_versions_dir.is_dir()
    assert settings.log_path.parent.is_dir()