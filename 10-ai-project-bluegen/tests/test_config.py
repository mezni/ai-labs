import pytest
from pydantic import ValidationError

from config import (
    LLMConfig,
    load_llm_config,
    load_pricing_config,
)


def test_llm_config_accepts_valid_configuration() -> None:
    config = LLMConfig(
        provider="openrouter",
        base_url="https://openrouter.ai/api/v1",
        model="openai/gpt-4o-mini",
        temperature=0.2,
        max_tokens=1000,
    )

    assert config.model == "openai/gpt-4o-mini"
    assert config.temperature == 0.2
    assert config.max_tokens == 1000


def test_llm_config_rejects_invalid_temperature() -> None:
    with pytest.raises(ValidationError):
        LLMConfig(
            provider="openrouter",
            base_url="https://openrouter.ai/api/v1",
            model="openai/gpt-4o-mini",
            temperature=5,
            max_tokens=1000,
        )


def test_llm_config_rejects_invalid_max_tokens() -> None:
    with pytest.raises(ValidationError):
        LLMConfig(
            provider="openrouter",
            base_url="https://openrouter.ai/api/v1",
            model="openai/gpt-4o-mini",
            temperature=0.2,
            max_tokens=0,
        )


def test_load_pricing_config() -> None:
    config = load_pricing_config()

    assert config.models


def test_pricing_contains_configured_model() -> None:
    llm_config = load_llm_config()
    pricing_config = load_pricing_config()

    assert llm_config.model in pricing_config.models