"""Unit tests for LLMClient using httpx.MockTransport (no network)."""

from __future__ import annotations

import httpx
import pytest

from src.llm.llm_client import LLMClient


def _request_json(request: httpx.Request) -> dict:
    import json

    return json.loads(request.content)


def _mock_transport(status: int = 200, payload: dict | None = None):
    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(status_code=status, json=payload or {}, request=request)

    return httpx.MockTransport(handler)


def _make_client(
    transport: httpx.MockTransport,
    env_openrouter,
    patch_llm_config,
    **kwargs,
) -> LLMClient:
    return LLMClient(transport=transport, **kwargs)


def test_init_reads_settings_from_config(env_openrouter, patch_llm_config):
    client = LLMClient(transport=_mock_transport())
    assert client.model == "minimax/MiniMax-M2"
    assert client.temperature == 0.3
    assert client.max_tokens == 4096
    assert (
        client.url
        == "https://openrouter.ai/api/v1/chat/completions"
    )
    client.close()


def test_init_model_override(env_openrouter, patch_llm_config):
    client = LLMClient(transport=_mock_transport(), model="anthropic/claude-3.5-haiku")
    assert client.model == "anthropic/claude-3.5-haiku"
    client.close()


def test_init_requires_api_key(monkeypatch, patch_llm_config):
    monkeypatch.delenv("OPENROUTER_API_KEY", raising=False)
    with pytest.raises(RuntimeError, match="OPENROUTER_API_KEY"):
        LLMClient(transport=_mock_transport())


def test_generate_returns_content(env_openrouter, patch_llm_config):
    transport = _mock_transport(payload={"choices": [{"message": {"content": "OK"}}]})
    client = _make_client(transport, env_openrouter, patch_llm_config)
    reply = client.generate([{"role": "user", "content": "ping"}])
    assert reply == "OK"
    client.close()


def test_generate_builds_payload(env_openrouter, patch_llm_config):
    captured = {}

    def handler(request: httpx.Request) -> httpx.Response:
        captured["json"] = _request_json(request)
        return httpx.Response(200, json={"choices": [{"message": {"content": "x"}}]}, request=request)

    client = LLMClient(transport=httpx.MockTransport(handler))
    messages = [{"role": "user", "content": "hi"}]
    client.generate(messages)

    payload = captured["json"]
    assert payload["model"] == "minimax/MiniMax-M2"
    assert payload["messages"] == messages
    assert payload["max_tokens"] == 4096
    assert payload["temperature"] == 0.3
    client.close()


def test_generate_kwargs_override_payload(env_openrouter, patch_llm_config):
    captured = {}

    def handler(request: httpx.Request) -> httpx.Response:
        captured["json"] = _request_json(request)
        return httpx.Response(200, json={"choices": [{"message": {"content": "x"}}]}, request=request)

    client = LLMClient(transport=httpx.MockTransport(handler))
    client.generate(
        [{"role": "user", "content": "hi"}],
        model="other/model",
        max_tokens=128,
        temperature=0.0,
    )

    payload = captured["json"]
    assert payload["model"] == "other/model"
    assert payload["max_tokens"] == 128
    assert payload["temperature"] == 0.0
    client.close()


def test_generate_raises_on_http_error(env_openrouter, patch_llm_config):
    client = LLMClient(transport=_mock_transport(status=429))
    with pytest.raises(httpx.HTTPStatusError):
        client.generate([{"role": "user", "content": "hi"}])
    client.close()


def test_classify_sends_system_then_user(env_openrouter, patch_llm_config):
    captured = {}

    def handler(request: httpx.Request) -> httpx.Response:
        captured["json"] = _request_json(request)
        return httpx.Response(200, json={"choices": [{"message": {"content": "intent"}}]}, request=request)

    client = LLMClient(transport=httpx.MockTransport(handler))
    result = client.classify("file a leave request", "You are a router.")

    assert result == "intent"
    messages = captured["json"]["messages"]
    assert messages[0] == {"role": "system", "content": "You are a router."}
    assert messages[1] == {"role": "user", "content": "file a leave request"}
    client.close()


def test_close_releases_client(env_openrouter, patch_llm_config):
    client = LLMClient(transport=_mock_transport())
    assert not client._client.is_closed
    client.close()
    assert client._client.is_closed