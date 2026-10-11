from copy import deepcopy
from types import SimpleNamespace
from unittest.mock import MagicMock

from support_agent import tool_calling


def make_response(content: str):
    message = SimpleNamespace(
        content=content,
        tool_calls=None,
        model_dump=MagicMock(),
    )
    return SimpleNamespace(choices=[SimpleNamespace(message=message)])


def test_follow_up_retains_conversation_history(monkeypatch):
    monkeypatch.setenv("OPENAI_MODEL", "test-model")

    client = MagicMock()
    captured_messages = []

    responses = [
        make_response(
            "Order #4821 has shipped. Estimated delivery is October 12, 2026."
        ),
        make_response("The estimated delivery date is October 12, 2026."),
    ]

    def fake_create(**kwargs):
        captured_messages.append(deepcopy(kwargs["messages"]))
        return responses.pop(0)

    client.chat.completions.create.side_effect = fake_create
    monkeypatch.setattr(
        tool_calling,
        "create_llm_client",
        lambda: client,
    )

    session = tool_calling.create_session()

    first_answer = tool_calling.handle_message(
        session,
        "Where is order #4821?",
    )
    second_answer = tool_calling.handle_message(
        session,
        "What about its delivery date?",
    )

    assert "4821" in first_answer
    assert "October 12, 2026" in second_answer

    second_turn_messages = captured_messages[1]
    assert any(
        message.get("role") == "user"
        and message.get("content") == "Where is order #4821?"
        for message in second_turn_messages
    )
    assert any(
        message.get("role") == "assistant" and message.get("content") == first_answer
        for message in second_turn_messages
    )


def test_sessions_are_isolated():
    session_a = tool_calling.create_session()
    session_b = tool_calling.create_session()

    session_a.messages.append({"role": "user", "content": "Private conversation A"})

    assert not any(
        message.get("content") == "Private conversation A"
        for message in session_b.messages
    )


def test_empty_message_is_rejected():
    session = tool_calling.create_session()

    try:
        tool_calling.handle_message(session, "   ")
    except ValueError:
        pass
    else:
        raise AssertionError("Expected ValueError")

    assert len(session.messages) == 1


from support_agent.tool_calling import create_session, handle_message


def main():
    session = create_session()

    print("Support agent ready. Type 'exit' to stop.")

    while True:
        user_message = input("You: ").strip()

        if user_message.lower() == "exit":
            break

        try:
            answer = handle_message(session, user_message)
            print(f"Agent: {answer}")
        except Exception as exc:
            print(f"Request failed: {exc}")


if __name__ == "__main__":
    main()
