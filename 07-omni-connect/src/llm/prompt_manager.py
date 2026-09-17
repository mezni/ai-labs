"""Prompt management and template loading."""
from typing import Dict, Any
from config.prompts import PROMPTS


class PromptManager:
    """Manages prompt templates.

    Templates live in config/prompts.py as a PROMPTS dict keyed by agent
    name, each entry with optional "system" and "user_template" keys.
    """

    def __init__(self) -> None:
        self.prompts: Dict[str, Any] = PROMPTS

    def get_prompt(self, prompt_name: str) -> Dict[str, Any]:
        """Get prompt by name."""
        return self.prompts.get(prompt_name, {})

    def format_prompt(self, template: str, **kwargs) -> str:
        """Format prompt template with variables."""
        return template.format(**kwargs)