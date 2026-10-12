import pytest

from ticket_triage.classifier import classify_ticket


def test_classifies_order_status_and_extracts_order_id():
    result = classify_ticket("My order #4821 has not arrived.")

    assert result == {
        "category": "order_status",
        "order_id": "4821",
        "needs_order_lookup": True,
    }


def test_order_without_id_does_not_trigger_lookup():
    result = classify_ticket("My order has not arrived.")

    assert result == {
        "category": "order_status",
        "order_id": None,
        "needs_order_lookup": False,
    }


def test_classifies_billing_ticket():
    result = classify_ticket("I was charged twice.")

    assert result["category"] == "billing"
    assert result["needs_order_lookup"] is False


def test_classifies_account_ticket():
    result = classify_ticket("I cannot log in to my account.")

    assert result["category"] == "account"


def test_unknown_ticket_is_general():
    result = classify_ticket("I have a question about your company.")

    assert result["category"] == "general"


@pytest.mark.parametrize("ticket_text", ["", "   ", "\n"])
def test_rejects_empty_ticket(ticket_text):
    with pytest.raises(ValueError):
        classify_ticket(ticket_text)


def test_rejects_non_string_input():
    with pytest.raises(ValueError):
        classify_ticket(None)
