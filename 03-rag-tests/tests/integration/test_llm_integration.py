"""Integration tests for LLMClient against the live OpenRouter API.

These tests are opt-in: they only run when RUN_INTEGRATION=1 and
OPENROUTER_API_KEY are set, so CI and local dev stays offline by default.

    RUN_INTEGRATION=1 uv run pytest tests/integration -m integration
"""

from __future__ import annotations

import os

import pytest

from src.llm.llm_client import LLMClient

pytestmark = [
    pytest.mark.integration,
    pytest.mark.skipif(
        os.getenv("RUN_INTEGRATION") != "1",
        reason="Set RUN_INTEGRATION=1 (and OPENROUTER_API_KEY) to run live API tests",
    ),
]


def test_chat_generate_live():
    client = LLMClient()
    try:
        reply = client.generate([{"role": "user", "content": "Reply with exactly: OK"}])
        assert isinstance(reply, str)
        assert reply.strip()
    finally:
        client.close()


def test_classify_live():
    client = LLMClient()
    try:
        label = client.classify("What is the leave policy?", "Reply with one word: LEAVE or ACCESS.")
        assert isinstance(label, str)
        assert label.strip()
    finally:
        client.close()