"""OpenRouter chat client for LLM generation."""

from __future__ import annotations

import os

import httpx
from dotenv import load_dotenv

from src.utils.config_loader import load_all_configs
from src.utils.logger import get_logger

load_dotenv()

logger = get_logger(__name__)

OPENROUTER_CHAT_URL = "https://openrouter.ai/api/v1/chat/completions"


class LLMClient:
    """Generate responses by calling the configured LLM."""

    def __init__(self) -> None:
        configs = load_all_configs()
        models = configs.get("llm", {}).get("models") or [{}]
        model_cfg = models[0]
        self.model = model_cfg.get("model", "gpt-4o-mini")
        self.url = model_cfg.get("openrouter_url", OPENROUTER_CHAT_URL)
        self.max_tokens = model_cfg.get("max_tokens", 4096)
        self.temperature = model_cfg.get("temperature", 0.3)
        self.provider = model_cfg.get("provider", "openrouter")

        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")

        self._client = httpx.Client(
            timeout=model_cfg.get("timeout_seconds", 120),
            headers={"Authorization": f"Bearer {api_key}"},
        )
        logger.debug(
            "LLMClient ready (model=%s, provider=%s, max_tokens=%d, temperature=%.2f)",
            self.model,
            self.provider,
            self.max_tokens,
            self.temperature,
        )

    def generate(self, messages: list[dict[str, str]]) -> str:
        """Send the chat messages to the LLM and return the reply."""
        logger.debug("Sending %d messages to %s", len(messages), self.model)
        response = self._client.post(
            self.url,
            json={
                "model": self.model,
                "max_tokens": self.max_tokens,
                "temperature": self.temperature,
                "messages": messages,
            },
        )
        response.raise_for_status()
        content = response.json()["choices"][0]["message"]["content"]
        logger.debug("Received response (%d chars)", len(content))
        return content

    def close(self) -> None:
        """Close the underlying HTTP client."""
        self._client.close()

    def __enter__(self) -> "LLMClient":
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()