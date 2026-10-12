import pytest
from pydantic import ValidationError
from ai_agent.classifier import classify_ticket_payload
from ai_agent.models import TicketCategory


def test_rule_1_refund_priority_over_order_status() -> None:
    """Rule 1: Refund requests take priority over order-status requests."""
    payload = {"text": "I need a refund for order #4821, it has not arrived"}
    res = classify_ticket_payload(payload)

    assert res.category == TicketCategory.REFUND_DISPUTE
    assert res.order_id == "4821"
    assert "Policy Rule 1 Applied" in res.assumptions[0]


def test_rule_2_order_status_missing_id() -> None:
    """Rule 2: Order-status requests may include a missing order ID."""
    payload = {"text": "Where is my order?"}
    res = classify_ticket_payload(payload)

    assert res.category == TicketCategory.ORDER_STATUS
    assert res.order_id is None
    assert res.needs_order_lookup is False
    assert res.needs_human_review is True  # Low confidence flags for review
    assert "Policy Rule 2 Applied" in res.assumptions[0]


def test_rule_3_fallback_to_general() -> None:
    """Rule 3: Unknown requests fall back to general."""
    payload = {"text": "What are your holiday store opening hours?"}
    res = classify_ticket_payload(payload)

    assert res.category == TicketCategory.GENERAL_INQUIRY
    assert res.confidence == 0.60
    assert "Policy Rule 3 Applied" in res.assumptions[0]


def test_rule_4_invalid_input_rejected() -> None:
    """Rule 4: Invalid input is rejected by the schema."""
    with pytest.raises(ValidationError):
        classify_ticket_payload({"text": "   "})

    with pytest.raises(ValidationError):
        classify_ticket_payload({"user_id": "usr_123"})  # Missing text key


def test_rule_5_operational_metadata_logging() -> None:
    """Rule 5: Verify metadata fields are computed without raw message leakage."""
    payload = {"text": "My order #4821 has not arrived."}
    res = classify_ticket_payload(payload)

    assert res.metadata.text_length == len(payload["text"])
    assert res.metadata.word_count == 6
    assert res.metadata.has_order_id_pattern is True
    assert not hasattr(res.metadata, "text")  # No raw message stored in metadata