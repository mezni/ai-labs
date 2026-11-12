import pytest

from support_ticket.exceptions import TicketNotFoundError
from support_ticket.models.ticket import TicketCategory, TicketStatus
from support_ticket.repositories.ticket_repository import TicketRepository
from support_ticket.services.ticket_service import TicketService


def test_create_ticket():

    service = TicketService(TicketRepository())

    ticket = service.create_ticket("My order #4821 has not arrived.")

    assert ticket.category == TicketCategory.ORDER_STATUS
    assert ticket.status == TicketStatus.OPEN


def test_reject_short_message():

    service = TicketService(TicketRepository())

    with pytest.raises(ValueError):
        service.create_ticket("Hi")


def test_ticket_not_found():

    service = TicketService(TicketRepository())

    with pytest.raises(TicketNotFoundError):
        service.get_ticket(999)
