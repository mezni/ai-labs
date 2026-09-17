"""Anthropic LLM client wrapper, configured from config/llm_config.yaml."""
import os
from typing import List, Dict, Any, Optional
import anthropic
from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)


class LLMClient:
    """Wrapper for Anthropic API calls.

    Model, temperature, and token settings come from config/llm_config.yaml
    by default. Pass explicit arguments to override the config for a specific
    instance (e.g. in tests) without editing the YAML file.

    The provider key is read, in order of preference, from ANTHROPIC_API_KEY,
    OPENROUTER_API_KEY, or OPENAI_API_KEY. LLM_BASE_URL is optional - unset,
    this is a plain Anthropic client; set it (e.g. to
    https://openrouter.ai/api/v1) to route through an Anthropic-API-compatible
    provider like OpenRouter. Model names in config/llm_config.yaml need to
    match whichever provider is active (OpenRouter uses provider-namespaced
    names like "anthropic/claude-3.5-haiku").
    """

    def __init__(self, model: Optional[str] = None, temperature: Optional[float] = None,
                 max_tokens: Optional[int] = None):
        config = load_yaml_config("config/llm_config.yaml")
        models_config = config.get("models", {})
        chat_config = models_config.get("chat", {})
        api_config = config.get("api", {})

        self.client = anthropic.Anthropic(
            api_key=os.getenv("ANTHROPIC_API_KEY")
            or os.getenv("OPENROUTER_API_KEY")
            or os.getenv("OPENAI_API_KEY"),
            base_url=os.getenv("LLM_BASE_URL") or None,
            timeout=api_config.get("timeout", 60),
            max_retries=api_config.get("max_retries", 2),
        )

        self.model = model or chat_config.get("model_name", "claude-3-5-haiku")
        self.temperature = temperature if temperature is not None else chat_config.get("temperature", 0.7)
        self.max_tokens = max_tokens if max_tokens is not None else chat_config.get("max_tokens", 2000)

        # Classification typically uses a lower temperature and a shorter
        # response budget than open-ended generation - config/llm_config.yaml
        # defines this separately under models.classification.
        classification_config = models_config.get("classification", {})
        self._classification_model = classification_config.get("model_name", self.model)
        self._classification_temperature = classification_config.get("temperature", 0.3)
        self._classification_max_tokens = classification_config.get("max_tokens", 500)

    def generate(self, messages: List[Dict[str, str]], **kwargs) -> str:
        """Generate completion from messages.

        Anthropic separates the system prompt from the conversation, so any
        role == "system" message is extracted and passed as the `system`
        parameter instead of part of the `messages` list.
        """
        system = "\n".join(
            msg["content"] for msg in messages if msg.get("role") == "system"
        ) or None
        conversation = [
            msg for msg in messages if msg.get("role") != "system"
        ]

        try:
            response = self.client.messages.create(
                model=kwargs.get("model", self.model),
                messages=conversation,
                system=system,
                temperature=kwargs.get("temperature", self.temperature),
                max_tokens=kwargs.get("max_tokens", self.max_tokens),
            )
            return "\n".join(
                block.text for block in response.content if block.type == "text"
            )
        except Exception as e:
            logger.error(f"LLM generation error: {e}")
            raise

    def classify(self, query: str, system_prompt: str) -> str:
        """Classify user query, using the classification model settings from config/llm_config.yaml."""
        messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": query}
        ]
        return self.generate(
            messages,
            model=self._classification_model,
            temperature=self._classification_temperature,
            max_tokens=self._classification_max_tokens
        )