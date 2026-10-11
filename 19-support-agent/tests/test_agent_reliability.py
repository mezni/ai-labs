import json
from unittest.mock import MagicMock

import pytest

from support_agent import tool_calling


def test_unknown_tool_is_rejected():
    result = tool_calling.execute_tool(
        "delete_order",
        json.dumps({"order_id": "4821"}),
    )

    assert result == {"error": "unsupported_tool"}


@pytest.mark.parametrize(
    "raw_arguments",
    [
        "{not-json",
        "null",
        "[]",
        json.dumps({"order_id": {"value": "4821"}}),
        json.dumps({"order_id": "4821", "extra": True}),
    ],
)
def test_invalid_order_tool_arguments_are_rejected(raw_arguments):
    result = tool_calling.execute_tool(
        "get_order_status",
        raw_arguments,
    )

    assert result == {"error": "invalid_tool_arguments"} or result == {
        "error": "invalid_json_arguments"
    }


def test_tool_exception_becomes_safe_error(monkeypatch):
    def broken_order_lookup(order_id: str):
        raise RuntimeError("Internal database details")

    monkeypatch.setattr(
        tool_calling,
        "get_order_status",
        broken_order_lookup,
    )

    result = tool_calling.execute_tool(
        "get_order_status",
        json.dumps({"order_id": "4821"}),
    )

    assert result == {"error": "tool_execution_failed"}
    assert "Internal database details" not in json.dumps(result)


def test_failed_model_call_does_not_commit_turn(monkeypatch):
    monkeypatch.setenv("OPENAI_MODEL", "test-model")

    session = tool_calling.create_session()
    original_messages = [dict(message) for message in session.messages]

    client = MagicMock()
    client.chat.completions.create.side_effect = RuntimeError("Simulated model failure")

    monkeypatch.setattr(
        tool_calling,
        "create_llm_client",
        lambda: client,
    )

    with pytest.raises(RuntimeError, match="Simulated model failure"):
        tool_calling.handle_message(
            session,
            "Where is order #4821?",
        )

    assert session.messages == original_messages
