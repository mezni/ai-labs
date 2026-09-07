"""Unit tests for ConversationMemory and QueryProcessor (src/memory.py)."""

from __future__ import annotations

import pytest

from src.memory import ConversationMemory, QueryProcessor


# -- ConversationMemory -------------------------------------------------------


def test_add_records_turns():
    memory = ConversationMemory(max_turns=3)
    memory.add("What is RAG?", "Retrieval-Augmented Generation.")
    assert len(memory) == 2
    assert memory.messages[0] == {"role": "user", "content": "What is RAG?"}


def test_rolling_limit_drops_oldest():
    memory = ConversationMemory(max_turns=2)
    memory.add("q1", "a1").add("q2", "a2").add("q3", "a3")
    # 3 turns -> kept 2 most recent = q2/a2 + q3/a3
    history = memory.history()
    assert [t["content"] for t in history] == ["q2", "a2", "q3", "a3"]


def test_history_respects_max_turns():
    memory = ConversationMemory(max_turns=5)
    memory.add("q1", "a1").add("q2", "a2").add("q3", "a3")
    recent = memory.history(max_turns=1)
    assert [t["content"] for t in recent] == ["q3", "a3"]


def test_to_text_renders_roles():
    memory = ConversationMemory(max_turns=3)
    memory.add("What is RAG?", "Retrieval augmented generation.")
    assert memory.to_text() == "User: What is RAG?\nAssistant: Retrieval augmented generation."


def test_add_ignores_empty_content():
    memory = ConversationMemory()
    memory.add("", "").add("q", "a")
    assert len(memory) == 2


def test_clear_resets():
    memory = ConversationMemory()
    memory.add("q", "a")
    memory.clear()
    assert not memory
    assert memory.history() == []


def test_invalid_max_turns_raises():
    with pytest.raises(ValueError):
        ConversationMemory(max_turns=0)


def test_history_negative_returns_empty():
    memory = ConversationMemory()
    memory.add("q", "a")
    assert memory.history(max_turns=0) == []


# -- QueryProcessor -----------------------------------------------------------


class FakeClient:
    def __init__(self, reply: str = "How much does the plan upgrade cost?") -> None:
        self.reply = reply
        self.calls: list[dict] = []

    def generate(self, messages, **kwargs):
        self.calls.append({"messages": messages, "kwargs": kwargs})
        return self.reply

    def close(self) -> None:
        pass


def _processor(client: FakeClient) -> QueryProcessor:
    processor = QueryProcessor()
    processor._client = lambda: client
    return processor


def test_process_with_history_rewrites(monkeypatch):
    # prompt template comes from the real config/prompts.yaml
    client = FakeClient()
    processor = _processor(client)
    memory = ConversationMemory()
    memory.add("How long is the return window?", "30 days.")
    result = processor.process("and what about refunds?", memory)

    assert result == "How much does the plan upgrade cost?"
    assert client.calls and client.calls[0]["kwargs"]["max_tokens"] == 128
    user_msg = client.calls[0]["messages"][-1]["content"]
    assert "30 days." in user_msg  # history reached the rewrite prompt


def test_process_without_history_returns_question():
    client = FakeClient()
    processor = _processor(client)
    assert processor.process("plain question", ConversationMemory()) == "plain question"
    assert client.calls == []  # no LLM call needed


def test_process_falls_back_on_llm_error():
    class BoomClient:
        def generate(self, messages, **kwargs):
            raise RuntimeError("boom")

        def close(self) -> None:
            pass

    processor = _processor(BoomClient())
    memory = ConversationMemory()
    memory.add("q", "a")
    assert processor.process("the question", memory) == "the question"


def test_process_strips_quotes():
    client = FakeClient(reply='"rewritten question"')
    processor = _processor(client)
    memory = ConversationMemory()
    memory.add("q", "a")
    assert processor.process("the question", memory) == "rewritten question"


def test_process_falls_back_on_empty_reply():
    client = FakeClient(reply="   ")
    processor = _processor(client)
    memory = ConversationMemory()
    memory.add("q", "a")
    assert processor.process("the question", memory) == "the question"