"""Conversation memory for the RAG copilot.

Memory and knowledge retrieval are different things:

- Memory answers "what were we talking about?" — the conversation history.
- RAG retrieval answers "what does the knowledge base say?" — the corpora.

This module provides:

- ``ConversationMemory`` — a bounded, rolling conversation history.
- ``QueryProcessor`` — rewrites the current question into a standalone
  retrieval query, resolving pronouns/references against the history. This is
  the "Query processing" step between the conversation and the retriever.

Query rewriting needs the chat LLM (network/config). If it is unavailable or
fails, the original question is used unchanged so the pipeline never breaks.
"""

from __future__ import annotations

from typing import Any

from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.utils import get_logger

logger = get_logger(__name__)

REWRITE_MAX_TOKENS = 128


class ConversationMemory:
    """Bounded rolling conversation history (user/assistant turns)."""

    def __init__(self, max_turns: int = 6) -> None:
        if max_turns < 1:
            raise ValueError("max_turns must be a positive integer")
        self.max_turns = max_turns
        self.messages: list[dict[str, str]] = []

    def add_user(self, content: str) -> "ConversationMemory":
        return self.add_message("user", content)

    def add_assistant(self, content: str) -> "ConversationMemory":
        return self.add_message("assistant", content)

    def add(self, user: str, assistant: str) -> "ConversationMemory":
        """Record one full turn (user question + assistant answer)."""
        return self.add_user(user).add_assistant(assistant)

    def add_message(self, role: str, content: str) -> "ConversationMemory":
        if not content:
            return self
        self.messages.append({"role": role, "content": content})
        if len(self.messages) > self.max_turns * 2:
            del self.messages[: len(self.messages) - self.max_turns * 2]
        return self

    def history(self, max_turns: int | None = None) -> list[dict[str, str]]:
        """Most recent max_turns user/assistant pairs as role/content messages."""
        limit = self.max_turns if max_turns is None else max_turns
        if limit < 1:
            return []
        return self.messages[-limit * 2 :]

    def to_text(self, max_turns: int | None = None) -> str:
        """Render the history as ``User: ...`` / ``Assistant: ...`` text."""
        return "\n".join(
            f"{turn['role'].capitalize()}: {turn['content']}"
            for turn in self.history(max_turns)
        )

    def clear(self) -> None:
        self.messages = []

    def __len__(self) -> int:
        return len(self.messages)

    def __bool__(self) -> bool:
        return bool(self.messages)


class QueryProcessor:
    """Rewrite the current question into a standalone retrieval query.

    Uses the conversation history so follow-ups like "how much is it?" or
    "and the return window?" retrieve against the full intent, not just the
    literal words of the latest question.
    """

    def __init__(self, prompt_version: str | None = None) -> None:
        self.prompt_version = prompt_version
        self.prompt_manager = PromptManager()

    def _client(self) -> LLMClient:
        return LLMClient()

    def process(self, question: str, memory: ConversationMemory) -> str:
        """Return a standalone query for retrieval.

        Falls back to the original question when the LLM is unavailable or the
        rewrite is empty.
        """
        history_text = memory.to_text()
        if not history_text:
            return question

        prompt = self.prompt_manager.get_prompt("query_rewrite", version=self.prompt_version)
        messages = self._rewrite_messages(prompt, history_text, question)
        if not messages:
            return question

        try:
            client = self._client()
            try:
                rewritten = client.generate(messages, max_tokens=REWRITE_MAX_TOKENS)
            finally:
                client.close()
        except Exception as exc:
            logger.warning("Query rewrite failed (%s); using original question", exc)
            return question

        rewritten = rewritten.strip().strip('"')
        logger.debug("Rewrote %r -> %r", question, rewritten)
        return rewritten if rewritten else question

    def _rewrite_messages(
        self, prompt: dict[str, Any], history_text: str, question: str
    ) -> list[dict[str, str]]:
        """Build the messages for the query_rewrite prompt (empty on missing template)."""
        user_template = prompt.get("user_template")
        if not user_template:
            logger.warning("query_rewrite prompt has no user_template")
            return []
        user = self.prompt_manager.format_prompt(
            user_template, history=history_text, question=question
        )
        messages: list[dict[str, str]] = []
        if prompt.get("system"):
            messages.append({"role": "system", "content": prompt["system"]})
        messages.append({"role": "user", "content": user})
        return messages