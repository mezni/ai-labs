import os
from dataclasses import dataclass

from openai import OpenAI

from utils.config_loader import DEFAULT_CONFIG_PATH, load_llm_config


class LLMConfigError(RuntimeError):
    pass


@dataclass
class LLMResponse:
    content: str
    model: str
    input_tokens: int | None = None
    output_tokens: int | None = None


class LLMClient:
    def __init__(self, config_path=DEFAULT_CONFIG_PATH):
        self.config = load_llm_config(config_path)

    @property
    def client(self) -> OpenAI:
        api_key = os.getenv(self.config["api_key_env"])
        if not api_key:
            raise LLMConfigError(
                f"{self.config['api_key_env']} is not set. Add it to your .env file."
            )
        return OpenAI(api_key=api_key, base_url=self.config["base_url"])

    def complete(self, messages: list[dict]) -> LLMResponse:
        response = self.client.chat.completions.create(
            model=self.config["model"],
            temperature=self.config["temperature"],
            response_format={"type": "json_object"},
            messages=messages,
        )

        usage = getattr(response, "usage", None)
        content = response.choices[0].message.content
        return LLMResponse(
            content=content,
            model=getattr(response, "model", self.config["model"]),
            input_tokens=usage.prompt_tokens if usage else None,
            output_tokens=usage.completion_tokens if usage else None,
        )