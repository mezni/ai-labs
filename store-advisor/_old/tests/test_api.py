import pytest
from fastapi.testclient import TestClient

from app.services.chat import DEFAULT_MODEL
from app.main import app

client = TestClient(app)


def test_health() -> None:
    r = client.get("/health")
    assert r.status_code == 200
    assert r.json() == {"status": "ok"}


def test_chat(monkeypatch) -> None:
    def fake_generate(messages, model=DEFAULT_MODEL, client=None) -> str:
        return "Hello there"

    monkeypatch.setattr("app.main.generate", fake_generate)
    r = client.post("/chat", json={"message": "hello"})
    assert r.status_code == 200
    assert r.json() == {"reply": "Hello there"}


def test_chat_default_model(monkeypatch) -> None:
    captured = {}

    def fake_generate(messages, model=DEFAULT_MODEL, client=None) -> str:
        captured["messages"] = messages
        captured["model"] = model
        return "hi"

    monkeypatch.setattr("app.main.generate", fake_generate)
    r = client.post("/chat", json={"message": "what is 2+2?"})
    assert r.status_code == 200
    assert captured["messages"] == [{"role": "user", "content": "what is 2+2?"}]
    assert captured["model"] is DEFAULT_MODEL


def test_chat_custom_model(monkeypatch) -> None:
    captured = {}

    def fake_generate(messages, model=DEFAULT_MODEL, client=None) -> str:
        captured["model"] = model
        return "hi"

    monkeypatch.setattr("app.main.generate", fake_generate)
    r = client.post("/chat", json={"message": "hi", "model": "openrouter/free"})
    assert r.status_code == 200
    assert captured["model"] == "openrouter/free"


def test_chat_llm_error(monkeypatch) -> None:
    def fake_generate(messages, model=DEFAULT_MODEL, client=None) -> str:
        raise RuntimeError("boom")

    monkeypatch.setattr("app.main.generate", fake_generate)
    r = client.post("/chat", json={"message": "hi"})
    assert r.status_code == 502
    assert r.json() == {"detail": "LLM request failed"}