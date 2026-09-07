"""Application entry point: hybrid retrieval and grounded generation."""
from __future__ import annotations

import os

import httpx
from dotenv import load_dotenv

from src.knowledge.embeddings import Embedder
from src.knowledge.knowledge_base import Chunk, KnowledgeBase
from src.knowledge.retriever import Retriever
from src.utils.config_loader import load_all_configs

load_dotenv()

__all__ = [
    "KnowledgeBase",
    "Embedder",
    "Chunk",
    "Retriever",
    "SYSTEM_PROMPT",
    "build_generation_prompt",
    "Generator",
    "answer",
]


SYSTEM_PROMPT = (
    "You answer questions using only the provided context. "
    "If the answer cannot be found in the context, "
    "say that you don't have enough information."
)


def build_generation_prompt(
    query: str,
    context: list[Chunk],
) -> list[dict[str, str]]:
    """Build a chat prompt combining retrieved context chunks with the query."""
    context_text = "\n\n".join(
        f"[Chunk {i}]\n{chunk['content']}"
        for i, chunk in enumerate(context, start=1)
    )
    user_prompt = f"CONTEXT:\n{context_text}\n\nQUESTION:\n{query}"
    return [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": user_prompt},
    ]


OPENROUTER_CHAT_URL = "https://openrouter.ai/api/v1/chat/completions"


class Generator:
    """Generate grounded answers by calling the configured LLM."""

    def __init__(self) -> None:
        configs = load_all_configs()
        models = configs.get("llm", {}).get("models") or [{}]
        model_cfg = models[0]
        self.model = model_cfg.get("model", "gpt-4o-mini")
        self.url = model_cfg.get("openrouter_url", OPENROUTER_CHAT_URL)
        self.max_tokens = model_cfg.get("max_tokens", 4096)
        self.temperature = model_cfg.get("temperature", 0.3)
        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")
        self._client = httpx.Client(
            timeout=model_cfg.get("timeout_seconds", 120),
            headers={"Authorization": f"Bearer {api_key}"},
        )

    def generate(self, messages: list[dict[str, str]]) -> str:
        """Send the chat messages to the LLM and return the reply."""
        response = self._client.post(
            self.url,
            json={
                "model": self.model,
                "max_tokens": self.max_tokens,
                "temperature": self.temperature,
                "messages": messages,
            },
        )
        response.raise_for_status()
        return response.json()["choices"][0]["message"]["content"]


def answer(
    query: str,
    retriever: Retriever,
    generator: Generator,
    top_k: int = 5,
) -> str:
    """Run the full RAG pipeline: retrieve, build a prompt, and generate an answer."""
    context = retriever.retrieve(query, top_k=top_k)
    prompt = build_generation_prompt(query, context)
    return generator.generate(prompt)