"""Shared test fixtures."""

import pytest


@pytest.fixture(autouse=True)
def _isolate_settings_env(monkeypatch):
    """Pin the environment so settings tests are deterministic."""
    for name in (
        "OPENROUTER_API_KEY",
        "OPENROUTER_BASE_URL",
        "OPENROUTER_EMBEDDING_MODEL",
        "OPENROUTER_MODEL",
        "CHUNK_SIZE",
        "CHUNK_OVERLAP",
        "TOP_K",
        "DATA_DIR",
        "INDEX_PATH",
        "METADATA_PATH",
        "DOCUMENT_STATE_PATH",
        "MANIFEST_PATH",
        "LOG_PATH",
    ):
        monkeypatch.delenv(name, raising=False)
    yield
