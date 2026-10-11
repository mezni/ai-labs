from unittest.mock import MagicMock, patch

import pytest

from support_agent.tool_calling import execute_tool


@patch("support_agent.tool_calling.create_llm_client")
@patch.dict("os.environ", {"OPENAI_MODEL": "test-model"})
def test_agent_stops_after_max_rounds(mock_create_client):
    client = MagicMock()
    mock_create_client.return_value = client

    call = MagicMock()
    call.id = "call_1"
    call.function.name = "get_order_status"
    call.function.arguments = '{"order_id":"4821"}'

    assistant_message = MagicMock()
    assistant_message.tool_calls = [call]
    assistant_message.content = None
    assistant_message.model_dump.return_value = {
        "role": "assistant",
        "tool_calls": [],
    }

    response = MagicMock()
    response.choices[0].message = assistant_message
    client.chat.completions.create.return_value = response

    from support_agent.tool_calling import handle_ticket

    with pytest.raises(RuntimeError, match="exceeded the limit"):
        handle_ticket("Where is order #4821?")

    assert client.chat.completions.create.call_count == 5


def test_executes_known_tool():
    result = execute_tool(
        "get_order_status",
        '{"order_id":"4821"}',
    )

    assert result["status"] == "shipped"


def test_rejects_unknown_tool():
    result = execute_tool("delete_order", "{}")

    assert result == {"error": "unsupported_tool"}


def test_rejects_invalid_json():
    result = execute_tool("get_order_status", "{invalid")

    assert result == {"error": "invalid_json_arguments"}


def test_rejects_invalid_arguments():
    result = execute_tool(
        "get_order_status",
        '{"order_id": 4821}',
    )

    assert result == {"error": "invalid_tool_arguments"}


@patch("support_agent.tool_calling.get_order_status")
def test_handles_tool_failure(mock_lookup):
    mock_lookup.side_effect = ValueError("simulated failure")

    result = execute_tool(
        "get_order_status",
        '{"order_id":"4821"}',
    )

    assert result == {"error": "tool_execution_failed"}
