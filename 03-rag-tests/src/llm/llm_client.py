"""OpenRouter chat client, configured from config/llm_config.yaml."""
import os
from typing import Dict, List, Optional

import httpx
from dotenv import load_dotenv

from src.utils import get_logger, load_config

load_dotenv()

logger = get_logger(__name__)


class LLMClient:
    """Generate responses by calling the LLM configured in llm_config.yaml.

    Chat, embedding, and reranker settings all come from config/llm_config.yaml.
    Requests are routed through the configured OpenRouter-compatible endpoints.
    """

    def __init__(
        self,
        model: Optional[str] = None,
        transport: Optional[httpx.BaseTransport] = None,
    ):
        config = load_config()
        chat_config = config.get("models", {}).get("chat", {})
        api_config = config.get("api", {})

        self.model = model or chat_config.get("model", "openai/gpt-4o-mini")
        self.temperature = chat_config.get("temperature", 0.3)
        self.max_tokens = chat_config.get("max_tokens", 4096)
        self.url = chat_config.get("openrouter_url", "https://openrouter.ai/api/v1/chat/completions")
        timeout = chat_config.get("timeout_seconds", api_config.get("timeout", 120))

        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")

        self._client = httpx.Client(
            transport=transport,
            timeout=timeout,
            headers={
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
            },
        )
        logger.debug(
            "LLMClient ready (model=%s, max_tokens=%d, temperature=%.2f)",
            self.model,
            self.max_tokens,
            self.temperature,
        )

    def generate(self, messages: List[Dict[str, str]], **kwargs) -> str:
        """Send the chat messages to the LLM and return the reply."""
        logger.debug("Sending %d messages to %s", len(messages), self.model)
        response = self._client.post(
            self.url,
            json={
                "model": kwargs.get("model", self.model),
                "messages": messages,
                "max_tokens": kwargs.get("max_tokens", self.max_tokens),
                "temperature": kwargs.get("temperature", self.temperature),
            },
        )
        response.raise_for_status()
        data = response.json()
        return data["choices"][0]["message"]["content"]

    def classify(self, query: str, system_prompt: str) -> str:
        """Classify user query using the chat model."""
        messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": query},
        ]
        return self.generate(messages)

    def close(self) -> None:
        self._client.close()