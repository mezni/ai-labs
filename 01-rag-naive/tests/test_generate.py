"""Tests for ``rag.generate`` RAG answer generation."""

from types import SimpleNamespace

import pytest
from pydantic import ValidationError

from rag.config import Settings
from rag.generate import (
    NO_CONTEXT_ANSWER,
    SYSTEM_PROMPT,
    GeneratedAnswer,
    GenerationError,
    Generator,
    build_generator,
    build_messages,
    build_user_prompt,
    serialize_context,
)
from rag.retrieve import RetrievalResult

DOCUMENT_ID = "billing_policy.pdf"


def _result(*, vector_id=1000, score=0.9, document_id=DOCUMENT_ID, page_start=12, page_end=12):
    return RetrievalResult(
        vector_id=vector_id,
        score=score,
        document_id=document_id,
        chunk_id=f"{document_id}::chunk::0",
        source=document_id,
        page_start=page_start,
        page_end=page_end,
        text="Refund requests are accepted within 30 days.",
    )


class StubCompletions:
    def __init__(self, responder):
        self.responder = responder
        self.calls = []

    def create(self, *, model, messages):
        self.calls.append({"model": model, "messages": messages})
        return self.responder(self.calls[-1])


class StubChatClient:
    def __init__(self, responder):
        self.chat = SimpleNamespace(completions=StubCompletions(responder))


def _response(content="Refunds are accepted within 30 days."):
    return SimpleNamespace(choices=[SimpleNamespace(message=SimpleNamespace(content=content))])


def _generator(client, model="minimax/minimax-m3", **kwargs):
    return Generator(client, model=model, **kwargs)


class TestContextSerialization:
    def test_serialize_context_single_result(self):
        result = _result(page_start=12, page_end=12)

        context = serialize_context([result])

        assert context == (
            "SOURCE: billing_policy.pdf\nPAGE: 12\n\nRefund requests are accepted within 30 days."
        )

    def test_serialize_context_page_range(self):
        context = serialize_context([_result(page_start=12, page_end=13)])

        assert "PAGE: 12-13" in context

    def test_serialize_context_multiple_results(self):
        first = _result()
        second = _result(vector_id=1001, page_start=13, page_end=13)

        context = serialize_context([first, second])

        assert context.startswith("SOURCE: billing_policy.pdf\nPAGE: 12")
        assert "\n\nSOURCE: billing_policy.pdf\nPAGE: 13" in context

    def test_build_user_prompt_includes_question_and_context(self):
        prompt = build_user_prompt("When can I get a refund?", "SOURCE: billing_policy.pdf")

        assert "QUESTION: When can I get a refund?" in prompt
        assert "CONTEXT:" in prompt
        assert "SOURCE: billing_policy.pdf" in prompt

    def test_build_messages_roles(self):
        messages = build_messages("question", "context")

        assert [message["role"] for message in messages] == ["system", "user"]
        assert messages[0]["content"] == SYSTEM_PROMPT
        assert "context" in messages[1]["content"]


class TestGenerator:
    def test_generate_returns_answer_and_sources(self):
        client = StubChatClient(lambda call: _response())
        generator = _generator(client)
        results = [_result()]

        answer = generator.generate("Can I refund after 30 days?", results)

        assert answer.answer == "Refunds are accepted within 30 days."
        assert answer.sources == results
        assert len(client.chat.completions.calls) == 1
        call = client.chat.completions.calls[0]
        assert call["model"] == "minimax/minimax-m3"
        assert call["messages"][0]["content"] == SYSTEM_PROMPT
        assert "Can I refund after 30 days?" in call["messages"][1]["content"]
        assert "billing_policy.pdf" in call["messages"][1]["content"]

    def test_generate_without_results_short_circuits(self):
        client = StubChatClient(lambda call: pytest.fail("no request expected"))
        generator = _generator(client)

        answer = generator.generate("question", [])

        assert answer.answer == NO_CONTEXT_ANSWER
        assert answer.sources == []
        assert client.chat.completions.calls == []

    def test_generate_uses_custom_system_prompt(self):
        client = StubChatClient(lambda call: _response())
        generator = _generator(client)
        custom = "Custom system instructions."

        generator.generate("question", [_result()], system_prompt=custom)

        assert client.chat.completions.calls[0]["messages"][0]["content"] == custom

    def test_generate_rejects_empty_question(self):
        client = StubChatClient(lambda call: _response())
        generator = _generator(client)

        with pytest.raises(ValueError, match="question"):
            generator.generate("   ", [_result()])

    class FlakyResponder:
        def __init__(self, failures):
            self.failures = failures
            self.attempts = 0

        def __call__(self, call):
            self.attempts += 1
            if self.attempts <= self.failures:
                raise RuntimeError("transient failure")
            return _response()

    def test_generate_retries_transient_failures(self, monkeypatch):
        sleeps = []
        monkeypatch.setattr("rag.generate.time.sleep", sleeps.append)
        responder = self.FlakyResponder(failures=2)
        client = StubChatClient(responder)
        generator = _generator(client, max_retries=3)

        answer = generator.generate("question", [_result()])

        assert responder.attempts == 3
        assert len(sleeps) == 2
        assert answer.answer == "Refunds are accepted within 30 days."

    def test_generate_raises_after_exhausting_retries(self, monkeypatch):
        monkeypatch.setattr("rag.generate.time.sleep", lambda seconds: None)
        responder = self.FlakyResponder(failures=99)
        client = StubChatClient(responder)
        generator = _generator(client, max_retries=2)

        with pytest.raises(GenerationError, match="3 attempts"):
            generator.generate("question", [_result()])

        assert responder.attempts == 3

    def test_generate_raises_when_no_choices(self):
        client = StubChatClient(lambda call: SimpleNamespace(choices=[]))
        generator = _generator(client)

        with pytest.raises(GenerationError, match="no choices"):
            generator.generate("question", [_result()])

    def test_generate_raises_when_content_missing(self):
        client = StubChatClient(lambda call: _response(content=None))
        generator = _generator(client)

        with pytest.raises(GenerationError, match="no text"):
            generator.generate("question", [_result()])

    def test_generator_requires_non_empty_model(self):
        with pytest.raises(ValueError, match="model"):
            Generator(StubChatClient(lambda call: None), model="")


class TestBuildGenerator:
    def test_build_generator_uses_openrouter_settings(self, monkeypatch):
        captured = {}

        class StubOpenAI:
            def __init__(self, **kwargs):
                captured.update(kwargs)

        monkeypatch.setattr("rag.generate.OpenAI", StubOpenAI)
        settings = Settings(
            openrouter_base_url="https://openrouter.example.com",
            openrouter_api_key="sk-test",
            openrouter_model="minimax/minimax-m3",
        )

        generator = build_generator(settings)

        assert captured["base_url"] == "https://openrouter.example.com"
        assert captured["api_key"] == "sk-test"
        assert generator.model == "minimax/minimax-m3"

    def test_build_generator_requires_model(self):
        with pytest.raises(ValueError, match="OPENROUTER_MODEL"):
            build_generator(Settings())


class TestGeneratedAnswerModel:
    def test_model_dump_shape(self):
        answer = GeneratedAnswer(answer="text", sources=[_result()])

        data = answer.model_dump(mode="json")

        assert set(data) == {"answer", "sources"}
        assert data["answer"] == "text"
        assert data["sources"] == [result.model_dump(mode="json") for result in [_result()]]

    def test_model_requires_answer(self):
        with pytest.raises(ValidationError):
            GeneratedAnswer(answer="", sources=[])
