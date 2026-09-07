"""Unit tests for the multi-version PromptManager."""

from __future__ import annotations

import pytest

from src.llm.prompt_manager import PromptManager

PROMPTS_YAML = """\
prompts:
  retrieval_query:
    default_version: v2
    user_template: |-
      CONTEXT:
      {context}

      QUESTION:
      {query}
    versions:
      v1:
        system: You answer from context only.
      v2:
        system: Answer using only the provided context.
  single:
    default_version: only
    user_template: "Say: {word}"
    versions:
      only:
        system: No system prompt needed.
"""


@pytest.fixture
def manager(tmp_path) -> PromptManager:
    prompts_file = tmp_path / "prompts.yaml"
    prompts_file.write_text(PROMPTS_YAML, encoding="utf-8")
    return PromptManager(str(prompts_file))


@pytest.fixture
def manager_with_empty_versions(tmp_path) -> PromptManager:
    prompts_file = tmp_path / "prompts.yaml"
    prompts_file.write_text(
        "prompts:\n  empty:\n    versions: {}\n",
        encoding="utf-8",
    )
    return PromptManager(str(prompts_file))


def test_loads_prompts_from_yaml(manager):
    assert set(manager.prompts) == {"retrieval_query", "single"}


def test_get_prompt_uses_default_version(manager):
    prompt = manager.get_prompt("retrieval_query")
    assert prompt["version"] == "v2"
    assert prompt["system"] == "Answer using only the provided context."
    assert "QUESTION:\n{query}" in prompt["user_template"]


def test_get_prompt_specific_version(manager):
    prompt = manager.get_prompt("retrieval_query", version="v1")
    assert prompt["version"] == "v1"
    assert prompt["system"] == "You answer from context only."


def test_get_prompt_unknown_version_falls_back_to_default(manager):
    prompt = manager.get_prompt("retrieval_query", version="nope")
    assert prompt["version"] == "v2"


def test_get_prompt_unknown_name_returns_empty(manager):
    assert manager.get_prompt("does_not_exist") == {}


def test_get_prompt_empty_versions_returns_empty(manager_with_empty_versions):
    assert manager_with_empty_versions.get_prompt("empty") == {}


def test_list_versions(manager):
    assert manager.list_versions("retrieval_query") == ["v1", "v2"]
    assert manager.list_versions("single") == ["only"]
    assert manager.list_versions("missing") == []


def test_format_prompt(manager):
    prompt = manager.get_prompt("retrieval_query")
    formatted = manager.format_prompt(
        prompt["user_template"], context="Leave is 30 days", query="How much leave?"
    )
    assert "Leave is 30 days" in formatted
    assert "How much leave?" in formatted


def test_format_prompt_missing_vars_returns_unformatted(manager):
    prompt = manager.get_prompt("retrieval_query")
    # Missing {query} -> returns the template untouched rather than raising.
    result = manager.format_prompt(prompt["user_template"], context="only")
    assert result == prompt["user_template"]


def test_format_prompt_ignores_extra_kwargs(manager):
    prompt = manager.get_prompt("single")
    formatted = manager.format_prompt(prompt["user_template"], word="hi", extra="ignored")
    assert formatted == "Say: hi"