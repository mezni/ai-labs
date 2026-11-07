import os
from dataclasses import dataclass

from dotenv import load_dotenv


load_dotenv()


@dataclass(frozen=True)
class LLMConfig:
    api_key: str
    model: str
    base_url: str | None
    timeout_seconds: float
    max_attempts: int

    @classmethod
    def from_env(cls) -> "LLMConfig":
        api_key = os.getenv("LLM_API_KEY", "").strip()
        model = os.getenv("LLM_MODEL", "").strip()

        if not api_key:
            raise ValueError("LLM_API_KEY is required.")

        if not model:
            raise ValueError("LLM_MODEL is required.")

        timeout = float(os.getenv("LLM_TIMEOUT_SECONDS", "30"))
        attempts = int(os.getenv("LLM_MAX_ATTEMPTS", "3"))

        if timeout <= 0:
            raise ValueError("Timeout must be positive.")

        if attempts < 1:
            raise ValueError("Max attempts must be at least 1.")

        base_url = os.getenv("LLM_BASE_URL", "").strip() or None

        return cls(
            api_key=api_key,
            model=model,
            base_url=base_url,
            timeout_seconds=timeout,
            max_attempts=attempts,
        )
