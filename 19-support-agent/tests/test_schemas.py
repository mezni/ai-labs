import pytest
from pydantic import ValidationError

from support_agent.schemas import TicketInput, TicketOutput


def test_valid_ticket():
    ticket = TicketInput(message="  Where is my order?  ")
    assert ticket.message == "Where is my order?"


@pytest.mark.parametrize("message", ["", "   ", "\n\t"])
def test_blank_ticket_is_rejected(message):
    with pytest.raises(ValidationError):
        TicketInput(message=message)


def test_ticket_length_limit():
    with pytest.raises(ValidationError):
        TicketInput(message="x" * 5001)


def test_invalid_category_is_rejected():
    with pytest.raises(ValidationError):
        TicketOutput(
            category="shipping",
            order_id=None,
            needs_order_lookup=False,
        )


def test_wrong_boolean_type_is_rejected():
    with pytest.raises(ValidationError):
        TicketOutput(
            category="order_status",
            order_id="4821",
            needs_order_lookup="yes",
        )


def test_output_serializes_to_json():
    result = TicketOutput(
        category="order_status",
        order_id="4821",
        needs_order_lookup=True,
    )

    data = result.model_dump()
    assert data["order_id"] == "4821"
    assert result.model_dump_json()
