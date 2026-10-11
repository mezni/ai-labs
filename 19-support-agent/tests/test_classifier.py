import pytest
from pydantic import ValidationError

from support_agent.classifier import classify_ticket


@pytest.mark.parametrize(
    ("message", "expected_category", "expected_order_id", "expected_lookup"),
    [
        (
            "My order #4821 has not arrived",
            "order_status",
            "4821",
            True,
        ),
        (
            "Where is my order?",
            "order_status",
            None,
            False,
        ),
        (
            "I need a refund for order #4821",
            "refund",
            "4821",
            False,
        ),
        (
            "How do I change my email?",
            "general",
            None,
            False,
        ),
        (
            "ORDER 4821 HAS SHIPPED",
            "order_status",
            "4821",
            True,
        ),
    ],
)
def test_classify_ticket(
    message,
    expected_category,
    expected_order_id,
    expected_lookup,
):
    result = classify_ticket(message)

    assert result.category == expected_category
    assert result.order_id == expected_order_id
    assert result.needs_order_lookup is expected_lookup


@pytest.mark.parametrize("message", ["", "   ", "\n\t"])
def test_rejects_blank_tickets(message):
    with pytest.raises(ValidationError):
        classify_ticket(message)


def test_rejects_non_string_input():
    with pytest.raises(ValidationError):
        classify_ticket(None)


def test_output_is_a_validated_model():
    result = classify_ticket("My order #4821 has arrived")

    assert result.category == "order_status"
    assert result.model_dump()["order_id"] == "4821"
