"""Centralized prompt assembly for the RAG pipeline."""

from __future__ import annotations

import os

from src.knowledge.knowledge_base import Chunk
from src.utils.config_loader import load_all_configs
from src.utils.logger import get_logger

logger = get_logger(__name__)

DEFAULT_SYSTEM_PROMPT = (
    "You answer questions using only the provided context. "
    "If the answer cannot be found in the context, "
    "say that you don't have enough information."
)

DEFAULT_USER_TEMPLATE = "CONTEXT:\n{context}\n\nQUESTION:\n{query}"

_LEGACY_VERSION = "default"


def build_generation_prompt(
    query: str,
    context: list[Chunk],
    system_prompt: str = DEFAULT_SYSTEM_PROMPT,
    user_template: str = DEFAULT_USER_TEMPLATE,
) -> list[dict[str, str]]:
    """Build a chat prompt combining retrieved context chunks with the query."""
    context_text = "\n\n".join(
        f"[Chunk {i}]\n{chunk['content']}"
        for i, chunk in enumerate(context, start=1)
    )
    user_prompt = user_template.format(context=context_text, query=query)
    return [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": user_prompt},
    ]


class PromptManager:
    """Load and resolve versioned prompts from config.

    Version resolution order: explicit `version` argument, then the
    `PROMPT_VERSION` environment variable, then the prompt's
    `default_version`.
    """

    def __init__(self, system_prompt: str | None = None) -> None:
        configs = load_all_configs()
        config_prompts = configs.get("prompts", {}).get("prompts", {})
        self._prompts = config_prompts

        default = self.get_prompt("retrieval_query")
        self.system_prompt = system_prompt or default["system"]
        self.user_template = default["user_template"]

    def _resolve_version(self, name: str, version: str | None) -> str:
        prompt = self._prompts.get(name)
        if prompt is None:
            raise ValueError(
                f"Unknown prompt {name!r}; available prompts: {sorted(self._prompts)}"
            )
        if "versions" not in prompt:
            return _LEGACY_VERSION
        env_version = os.getenv("PROMPT_VERSION")
        resolved = version or env_version or prompt.get("default_version")
        if resolved is None:
            raise ValueError(
                f"Prompt {name!r} has no default_version and none was requested"
            )
        if resolved not in prompt["versions"]:
            raise ValueError(
                f"Unknown version {resolved!r} for prompt {name!r}; "
                f"available versions: {sorted(prompt['versions'])}"
            )
        return resolved

    def get_prompt(
        self,
        name: str,
        version: str | None = None,
    ) -> dict[str, str]:
        """Return the {system, user_template, version} for a named prompt."""
        prompt = self._prompts.get(name)
        if prompt is None:
            raise ValueError(
                f"Unknown prompt {name!r}; available prompts: {sorted(self._prompts)}"
            )

        resolved = self._resolve_version(name, version)
        if resolved == _LEGACY_VERSION:
            entry = prompt
        else:
            entry = prompt["versions"][resolved]
        return {
            "system": entry.get("system", ""),
            "user_template": entry.get("user_template", ""),
            "version": resolved,
        }

    def format_prompt(
        self,
        name: str,
        version: str | None = None,
        **kwargs: str,
    ) -> list[dict[str, str]]:
        """Format a versioned prompt with the given fields into chat messages.

        `kwargs` are used to fill the prompt's user template fields.
        """
        prompt = self.get_prompt(name, version=version)
        user_prompt = prompt["user_template"].format(**kwargs)
        logger.info("Formatted prompt %r@%s", name, prompt["version"])
        return [
            {"role": "system", "content": prompt["system"]},
            {"role": "user", "content": user_prompt},
        ]

    def build_generation_prompt(
        self,
        query: str,
        context: list[Chunk],
        version: str | None = None,
    ) -> list[dict[str, str]]:
        """Assemble the system + user messages for answering a query with context."""
        prompt = self.get_prompt("retrieval_query", version=version)
        messages = build_generation_prompt(
            query,
            context,
            system_prompt=prompt["system"],
            user_template=prompt["user_template"],
        )
        logger.info(
            "Built generation prompt %r with %d context chunks using %r@%s",
            query,
            len(context),
            "retrieval_query",
            prompt["version"],
        )
        return messages