from support_ticket.models.ticket import Ticket
from support_ticket.repositories.ticket_repository import TicketRepository
from support_ticket.services.ticket_service import TicketService


def test_create_order_status_ticket():

    repository = TicketRepository()
    service = TicketService(repository)

    ticket = service.create_ticket("My order #4821 has not arrived.")

    assert ticket.category == "order_status"
    assert ticket.status == "open"
    assert ticket.ticket_id == 1


def test_repository_retrieves_ticket():

    repository = TicketRepository()

    ticket = Ticket(
        ticket_id=1,
        message="My order has not arrived.",
        category="general",
    )

    repository.save(ticket)

    result = repository.get_by_id(1)

    assert result == ticket
