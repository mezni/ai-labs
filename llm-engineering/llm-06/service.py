from .provider import OpenAICompatibleProvider


class LLMService:
    def __init__(self, provider: OpenAICompatibleProvider):
        self.provider = provider

    async def answer(self, question: str) -> str:
        if not question.strip():
            raise ValueError("Question cannot be empty.")

        result = await self.provider.generate(question)

        return result.content
