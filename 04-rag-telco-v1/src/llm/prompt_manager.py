"""Prompt management and template loading."""
from typing import Any

from src.utils import get_logger, load_config

logger = get_logger(__name__)

PROMPTS_PATH = "config/prompts.yaml"


class PromptManager:
    """Manages multi-version prompt templates.

    Templates live in config/prompts.yaml as a ``prompts`` dict keyed by prompt
    name. Each prompt entry supports:

      default_version: name of the active version
      user_template:   template with {placeholders}, e.g. CONTEXT/{context}
      versions:
        v1:            per-version settings (e.g. system prompt)

    Versions can override any top-level field, so version-specific system
    prompts (or even user templates) are merged over the shared defaults.
    """

    def __init__(self, prompts_path: str = PROMPTS_PATH) -> None:
        config = load_config(prompts_path)
        self.prompts: dict[str, Any] = config.get("prompts", {})
        logger.debug(
            "PromptManager ready (%d prompts)", len(self.prompts)
        )

    def get_prompt(self, prompt_name: str, version: str | None = None) -> dict[str, Any]:
        """Get prompt by name, resolved to a specific (or default) version.

        Falls back to ``default_version``, then to the first available version
        if the requested one is missing. Returns ``{}`` for unknown prompts.
        """
        entry = self.prompts.get(prompt_name)
        if not entry:
            logger.warning("Unknown prompt %r", prompt_name)
            return {}

        versions = entry.get("versions", {}) or {}
        requested = version or entry.get("default_version")
        if requested not in versions:
            if version is not None:
                logger.warning(
                    "Prompt %r has no version %r; using default",
                    prompt_name,
                    version,
                )
            requested = entry.get("default_version") or next(iter(versions), None)
        if requested is None or requested not in versions:
            logger.warning("Prompt %r has no usable versions", prompt_name)
            return {}

        merged: dict[str, Any] = {
            "version": requested,
            "user_template": entry.get("user_template", ""),
        }
        for key, value in entry.items():
            if key not in ("versions", "default_version", "name"):
                merged.setdefault(key, value)
        merged.update(versions[requested])
        merged.setdefault("user_template", entry.get("user_template", ""))
        return merged

    def list_versions(self, prompt_name: str) -> list[str]:
        """Return the available versions for a prompt (may be empty)."""
        entry = self.prompts.get(prompt_name, {})
        versions = entry.get("versions", {})
        return list(versions.keys()) or (
            [entry["default_version"]] if entry.get("default_version") else []
        )

    def format_prompt(self, template: str, **kwargs) -> str:
        """Format prompt template with variables.

        Missing placeholders are left untouched (with a warning) instead of
        raising, so an incomplete context never crashes the caller.
        """
        try:
            return template.format(**kwargs)
        except (KeyError, IndexError) as exc:
            logger.warning("Prompt formatting failed (%s); returning unformatted", exc)
            return template