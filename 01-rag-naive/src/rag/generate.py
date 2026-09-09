"""Answer generation: build a RAG prompt and call a chat model through OpenRouter."""

from __future__ import annotations

import logging
import time
from collections.abc import Sequence

from openai import OpenAI
from pydantic import BaseModel, ConfigDict, Field

from rag.config import Settings
from rag.retrieve import RetrievalResult

logger = logging.getLogger(__name__)

DEFAULT_MAX_RETRIES = 3
DEFAULT_RETRY_DELAY = 1.0

SYSTEM_PROMPT = (
    "You are a document assistant. Answer the user's question using ONLY the "
    "retrieved context below. Do not invent facts that are not present in the "
    "context, and preserve important policy conditions and numbers exactly. "
    "If the context does not contain the answer, clearly state that the answer "
    "is not present in the provided documents. Cite the source documents where "
    "applicable."
)

NO_CONTEXT_ANSWER = (
    "I could not find a relevant passage in the provided documents, so I cannot "
    "answer this question."
)


class GenerationError(RuntimeError):
    """Raised when a chat completion request fails or returns no text."""


class GeneratedAnswer(BaseModel):
    """The model's answer plus the sources it was generated from."""

    model_config = ConfigDict(frozen=True)

    answer: str = Field(min_length=1)
    sources: list[RetrievalResult]


def serialize_context(results: Sequence[RetrievalResult]) -> str:
    """Serialize retrieved chunks into SOURCE/PAGE-aware context blocks."""
    blocks: list[str] = []
    for result in results:
        if result.page_start == result.page_end:
            page = str(result.page_start)
        else:
            page = f"{result.page_start}-{result.page_end}"
        blocks.append(f"SOURCE: {result.source}\nPAGE: {page}\n\n{result.text}")
    return "\n\n".join(blocks)


def build_user_prompt(question: str, context: str) -> str:
    """Combine the user question with the serialized retrieval context."""
    return f"QUESTION: {question}\n\nCONTEXT:\n{context}"


def build_messages(
    question: str, context: str, *, system_prompt: str = SYSTEM_PROMPT
) -> list[dict[str, str]]:
    """Return the system/user message list for the chat completions call."""
    return [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": build_user_prompt(question, context)},
    ]


class Generator:
    """Ask a chat model (MiniMax) to answer using retrieved context."""

    def __init__(
        self,
        client: OpenAI,
        *,
        model: str,
        max_retries: int = DEFAULT_MAX_RETRIES,
        retry_delay: float = DEFAULT_RETRY_DELAY,
    ) -> None:
        if not model:
            raise ValueError("model must be a non-empty chat model id")
        if max_retries < 0:
            raise ValueError("max_retries must be >= 0")
        if retry_delay < 0:
            raise ValueError("retry_delay must be >= 0")
        self.client = client
        self.model = model
        self.max_retries = max_retries
        self.retry_delay = retry_delay

    def generate(
        self,
        question: str,
        results: Sequence[RetrievalResult],
        *,
        system_prompt: str = SYSTEM_PROMPT,
    ) -> GeneratedAnswer:
        """Generate an answer from ``results``, or short-circuit without a call."""
        if not results:
            return GeneratedAnswer(answer=NO_CONTEXT_ANSWER, sources=[])
        if not question.strip():
            raise ValueError("question must not be empty")

        context = serialize_context(results)
        messages = build_messages(question, context, system_prompt=system_prompt)
        response = self._request(messages)
        return GeneratedAnswer(answer=self._extract_content(response), sources=list(results))

    def _request(self, messages: Sequence[dict[str, str]]):
        last_error: Exception | None = None
        for attempt in range(self.max_retries + 1):
            try:
                return self.client.chat.completions.create(model=self.model, messages=messages)
            except Exception as exc:
                last_error = exc
                logger.warning("Chat completion failed (attempt %s): %s", attempt + 1, exc)
                if attempt < self.max_retries:
                    time.sleep(self.retry_delay * (2**attempt))
        raise GenerationError(
            f"chat completion failed after {self.max_retries + 1} attempts: {last_error}"
        ) from last_error

    @staticmethod
    def _extract_content(response) -> str:
        choices = getattr(response, "choices", None)
        if not isinstance(choices, list) or not choices:
            raise GenerationError("chat completion response contained no choices")
        message = getattr(choices[0], "message", None)
        content = getattr(message, "content", None) if message is not None else None
        if not isinstance(content, str) or not content:
            raise GenerationError("chat completion response contained no text")
        return content


def build_generator(settings: Settings) -> Generator:
    """Create a :class:`Generator` from the configured OpenRouter settings."""
    if not settings.openrouter_model:
        raise ValueError("OPENROUTER_MODEL is not configured")
    if not settings.openrouter_api_key:
        logger.warning("OPENROUTER_API_KEY is not set; generation requests will fail")
    client = OpenAI(
        base_url=settings.openrouter_base_url,
        api_key=settings.openrouter_api_key or None,
    )
    return Generator(client, model=settings.openrouter_model)
