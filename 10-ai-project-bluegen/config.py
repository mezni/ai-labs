from pathlib import Path

import yaml
from pydantic import BaseModel, Field
from pydantic_settings import BaseSettings, SettingsConfigDict


CONFIG_PATH = Path("config/llm.yaml")
PRICING_CONFIG_PATH = Path("config/pricing.yaml")


class Settings(BaseSettings):
    openrouter_api_key: str = Field(min_length=1)

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
    )


class LLMConfig(BaseModel):
    provider: str
    base_url: str
    model: str
    temperature: float = Field(ge=0.0, le=2.0)
    max_tokens: int = Field(gt=0)


class ModelPricingConfig(BaseModel):
    input_cost_per_million_tokens: float = Field(
        ge=0.0
    )
    output_cost_per_million_tokens: float = Field(
        ge=0.0
    )


class PricingConfig(BaseModel):
    models: dict[str, ModelPricingConfig]


def load_settings() -> Settings:
    return Settings()


def load_llm_config() -> LLMConfig:
    if not CONFIG_PATH.exists():
        raise FileNotFoundError(
            f"Configuration file not found: {CONFIG_PATH}"
        )

    with CONFIG_PATH.open("r", encoding="utf-8") as file:
        data = yaml.safe_load(file)

    if not data:
        raise ValueError("LLM configuration is empty.")

    return LLMConfig.model_validate(data)


def load_pricing_config() -> PricingConfig:
    if not PRICING_CONFIG_PATH.exists():
        raise FileNotFoundError(
            f"Pricing configuration not found: "
            f"{PRICING_CONFIG_PATH}"
        )

    with PRICING_CONFIG_PATH.open(
        "r",
        encoding="utf-8",
    ) as file:
        data = yaml.safe_load(file)

    if not data:
        raise ValueError(
            "Pricing configuration is empty."
        )

    return PricingConfig.model_validate(data)