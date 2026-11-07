import asyncio
import random
from collections.abc import AsyncIterator

from openai import (
    APIConnectionError,
    APITimeoutError,
    AsyncOpenAI,
    InternalServerError,
    RateLimitError,
)

from .config import LLMConfig
from .models import LLMResult


class OpenAICompatibleProvider:
    def __init__(self, config: LLMConfig):
        self.config = config

        client_args = {
            "api_key": config.api_key,
            "timeout": config.timeout_seconds,
            # Disable SDK retries because this class owns retry policy.
            "max_retries": 0,
        }

        if config.base_url:
            client_args["base_url"] = config.base_url

        self.client = AsyncOpenAI(**client_args)

    async def generate(self, prompt: str) -> LLMResult:
        for attempt in range(self.config.max_attempts):
            try:
                response = await self.client.chat.completions.create(
                    model=self.config.model,
                    messages=[
                        {
                            "role": "system",
                            "content": "You are a helpful telecom support assistant.",
                        },
                        {
                            "role": "user",
                            "content": prompt,
                        },
                    ],
                    temperature=0.2,
                    max_completion_tokens=300,
                )

                choice = response.choices[0]
                usage = response.usage

                return LLMResult(
                    content=choice.message.content or "",
                    model=response.model,
                    input_tokens=usage.prompt_tokens if usage else 0,
                    output_tokens=usage.completion_tokens if usage else 0,
                    finish_reason=choice.finish_reason,
                )

            except (
                RateLimitError,
                APITimeoutError,
                APIConnectionError,
                InternalServerError,
            ):
                if attempt + 1 >= self.config.max_attempts:
                    raise

                delay = min(8.0, 1.0 * (2**attempt))
                delay = random.uniform(0, delay)

                await asyncio.sleep(delay)

    async def stream(self, prompt: str) -> AsyncIterator[str]:
        stream = await self.client.chat.completions.create(
            model=self.config.model,
            messages=[
                {
                    "role": "system",
                    "content": "You are a helpful telecom support assistant.",
                },
                {
                    "role": "user",
                    "content": prompt,
                },
            ],
            temperature=0.2,
            max_completion_tokens=300,
            stream=True,
        )

        async for chunk in stream:
            if not chunk.choices:
                continue

            content = chunk.choices[0].delta.content

            if content:
                yield content

    async def close(self) -> None:
        await self.client.close()
