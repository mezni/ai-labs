import pytest

from config import LLMConfig


def test_load_valid_configuration(monkeypatch):
    monkeypatch.setenv("LLM_API_KEY", "test-key")
    monkeypatch.setenv("LLM_MODEL", "test-model")
    monkeypatch.setenv("LLM_TIMEOUT_SECONDS", "20")
    monkeypatch.setenv("LLM_MAX_ATTEMPTS", "3")

    config = LLMConfig.from_env()

    assert config.api_key == "test-key"
    assert config.model == "test-model"
    assert config.timeout_seconds == 20
    assert config.max_attempts == 3


def test_missing_api_key_is_rejected(monkeypatch):
    monkeypatch.setenv("LLM_API_KEY", "")
    monkeypatch.setenv("LLM_MODEL", "test-model")

    with pytest.raises(ValueError):
        LLMConfig.from_env()


def test_invalid_timeout_is_rejected(monkeypatch):
    monkeypatch.setenv("LLM_API_KEY", "test-key")
    monkeypatch.setenv("LLM_MODEL", "test-model")
    monkeypatch.setenv("LLM_TIMEOUT_SECONDS", "0")

    with pytest.raises(ValueError):
        LLMConfig.from_env()
