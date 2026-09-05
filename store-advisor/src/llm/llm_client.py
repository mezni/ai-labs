import os
from functools import lru_cache
from pathlib import Path
from typing import Dict, List, Optional, Union

from dotenv import load_dotenv
from openai import OpenAI
from pydantic import BaseModel, Field

from src.utils.config_loader import get_config
from src.utils.logger import setup_logger

load_dotenv()

logger = setup_logger(__name__)

CONFIG_PATH = Path(__file__).resolve().parents[2] / "config" / "llm_config.yaml"


class ProviderConfig(BaseModel):
    api_key_env: str
    base_url: str

    @property
    def api_key(self) -> str:
        key = os.getenv(self.api_key_env)
        if not key:
            raise RuntimeError(f"Environment variable '{self.api_key_env}' is not set.")
        return key


class ModelConfig(BaseModel):
    provider: str
    name: str
    temperature: float = Field(default=0.7, ge=0.0, le=2.0)
    max_tokens: int = Field(default=2048, gt=0)
    timeout: float = Field(default=60.0, gt=0)


class LLMConfig(BaseModel):
    providers: Dict[str, ProviderConfig]
    models: Dict[str, ModelConfig]
    default_model: str


@lru_cache(maxsize=1)
def load_llm_config(path: Path = CONFIG_PATH) -> LLMConfig:
    """Load and cache the LLM configuration."""
    return LLMConfig.model_validate(get_config(path, "llm"))


def get_model_config(model_key: Optional[str] = None) -> ModelConfig:
    cfg = load_llm_config()
    key = model_key or cfg.default_model
    if key not in cfg.models:
        raise KeyError(f"Unknown model key '{key}'. Available: {list(cfg.models.keys())}")

    model = cfg.models[key]
    if model.provider not in cfg.providers:
        raise KeyError(f"Unknown provider '{model.provider}' specified for model '{key}'")
    return model


class LLMClient:
    def __init__(
        self,
        model_key: Optional[str] = None,
        client: Optional[OpenAI] = None,
        **overrides: Union[float, int],
    ) -> None:
        raw_cfg = get_model_config(model_key)
        # Re-validate overrides against ModelConfig rules
        self._cfg = ModelConfig(**{**raw_cfg.model_dump(), **overrides}) if overrides else raw_cfg

        self.model = self._cfg.name
        self.temperature = self._cfg.temperature
        self.max_tokens = self._cfg.max_tokens
        self.timeout = self._cfg.timeout
        self._client = client or self._build_client(raw_cfg.provider)

        logger.info(
            "LLMClient initialized: model=%s temperature=%s max_tokens=%s",
            self.model, self.temperature, self.max_tokens
        )

    def _build_client(self, provider_name: str) -> OpenAI:
        provider = load_llm_config().providers[provider_name]
        logger.debug("Building OpenAI client for provider=%s base_url=%s", provider_name, provider.base_url)
        return OpenAI(
            base_url=provider.base_url,
            api_key=provider.api_key,
        )

    def generate(self, messages: List[Dict[str, str]], **kwargs) -> str:
        """Generate completion from messages."""
        params = {
            "model": self.model,
            "messages": messages,
            "temperature": self.temperature,
            "max_tokens": self.max_tokens,
            "timeout": self.timeout,
        }
        params.update(kwargs)
        logger.debug("Calling model=%s with %d messages", self.model, len(messages))

        response = self._client.chat.completions.create(**params)
        choice = response.choices[0]
        content = choice.message.content

        if content is None:
            raise RuntimeError(
                f"Model returned empty content (finish_reason: '{choice.finish_reason}')."
            )

        logger.debug("Received %d chars from model", len(content))
        return content


def get_client(model_key: Optional[str] = None) -> LLMClient:
    return LLMClient(model_key)