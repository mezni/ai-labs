from functools import lru_cache

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    app_name: str = "Telecom Policy Assistant API"
    app_env: str = "development"

    database_url: str = "postgresql+psycopg2://policy:policy@localhost:5432/policy_assistant"

    openrouter_api_key: str = ""
    jwt_secret: str = ""

    langfuse_public_key: str = ""
    langfuse_secret_key: str = ""
    langfuse_host: str = "http://localhost:3002"

    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8", extra="ignore")


@lru_cache
def get_settings() -> Settings:
    return Settings()
