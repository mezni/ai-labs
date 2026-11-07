import asyncio

from config import LLMConfig
from provider import OpenAICompatibleProvider
from service import LLMService


async def main():
    config = LLMConfig.from_env()
    provider = OpenAICompatibleProvider(config)
    service = LLMService(provider)

    try:
        result = await provider.generate(
            "Explain why a telecom customer's internet bill might increase."
        )

        print("Answer:")
        print(result.content)

        print("\nUsage:")
        print(f"Input tokens: {result.input_tokens}")
        print(f"Output tokens: {result.output_tokens}")
        print(f"Total tokens: {result.total_tokens}")

        print("\nStreaming example:")

        async for chunk in provider.stream(
            "Give three steps to troubleshoot slow internet."
        ):
            print(chunk, end="", flush=True)

        print()

    finally:
        await provider.close()


if __name__ == "__main__":
    asyncio.run(main())
