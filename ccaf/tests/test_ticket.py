from ccaf.ticket import Ticket


def test_ticket_creation():
    ticket = Ticket(
        ticket_id="TICKET-001",
        message="My internet connection has stopped working.",
    )
    assert ticket.ticket_id == "TICKET-001"
    assert ticket.message == "My internet connection has stopped working."
