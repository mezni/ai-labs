import pytest
from pydantic import ValidationError

from ai_agent.classifier import classify_ticket_payload
from ai_agent.models import TicketCategory


def test_handles_unexpected_keys() -> None:
    raw_payload = {
        "text": "My order #4821 has not arrived.",
        "unexpected_key": "some_value",
        "random_metadata": 12345,
    }
    result = classify_ticket_payload(raw_payload)
    assert result.category == TicketCategory.ORDER_STATUS
    assert result.order_id == "4821"


def test_raises_error_on_missing_required_fields() -> None:
    raw_payload = {"user_id": "usr_99"}  # Missing 'text'
    with pytest.raises(ValidationError) as exc_info:
        classify_ticket_payload(raw_payload)

    assert "text" in str(exc_info.value)


def test_raises_error_on_empty_string_value() -> None:
    raw_payload = {"text": "   "}
    with pytest.raises(ValidationError):
        classify_ticket_payload(raw_payload)
