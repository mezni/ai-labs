"""Unit tests for src.llm.prompt_manager."""
import pytest

from src.llm.prompt_manager import PromptManager

PROMPTS = """
prompts:
  rag_answer:
    default_version: v1
    user_template: |
      CONTEXT:
      {context}

      QUESTION:
      {question}
    versions:
      v1:
        system: system-one
      v2:
        system: system-two
  no_versions:
    default_version: v1
"""


@pytest.fixture
def manager(tmp_path):
    prompts_path = tmp_path / "prompts.yaml"
    prompts_path.write_text(PROMPTS, encoding="utf-8")
    return PromptManager(str(prompts_path))


def test_list_versions_available(manager):
    assert manager.list_versions("rag_answer") == ["v1", "v2"]


def test_get_prompt_default_version(manager):
    prompt = manager.get_prompt("rag_answer")
    assert prompt["version"] == "v1"
    assert prompt["system"] == "system-one"
    assert "{context}" in prompt["user_template"]


def test_get_prompt_specific_version(manager):
    prompt = manager.get_prompt("rag_answer", version="v2")
    assert prompt["version"] == "v2"
    assert prompt["system"] == "system-two"


def test_get_prompt_unknown_name_returns_empty(manager):
    assert manager.get_prompt("does_not_exist") == {}


def test_get_prompt_unknown_version_falls_back_to_default(manager):
    prompt = manager.get_prompt("rag_answer", version="v9")
    assert prompt["version"] == "v1"
    assert prompt["system"] == "system-one"


def test_get_prompt_no_versions_returns_empty(manager):
    assert manager.get_prompt("no_versions") == {}


def test_format_prompt(manager):
    template = "CONTEXT: {context}\nQ: {question}"
    formatted = manager.format_prompt(template, context="docs", question="how")
    assert formatted == "CONTEXT: docs\nQ: how"


def test_format_prompt_missing_placeholder_returns_template(manager):
    template = "CONTEXT: {context}"
    assert manager.format_prompt(template, question="how") == template