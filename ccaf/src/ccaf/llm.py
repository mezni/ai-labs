import os
from dotenv import load_dotenv
from anthropic import Anthropic

load_dotenv()


def get_api_key() -> str:
    api_key = os.getenv("ANTHROPIC_API_KEY")

    if not api_key:
        raise ValueError("ANTHROPIC_API_KEY is not configured")

    return api_key


def get_model() -> str:
    return os.getenv("ANTHROPIC_MODEL", "claude-haiku-4-5-20251001")


class ClaudeClient:
    def __init__(self) -> None:
        self._client = Anthropic(
            api_key=get_api_key(),
            timeout=30.0,
            max_retries=2,
        )
        self._model = get_model()

    def generate(self, prompt: str) -> str:
        response = self._client.messages.create(
            model=self._model,
            max_tokens=1000,
            messages=[{"role": "user", "content": prompt}],
        )
        for block in response.content:
            if block.type == "text":
                return block.text
        raise RuntimeError("Claude returned no text response")
