from support_ticket.models.ticket import Ticket


class TicketRepository:
    def __init__(self) -> None:
        self._tickets: dict[int, Ticket] = {}

    def save(self, ticket: Ticket) -> Ticket:
        self._tickets[ticket.ticket_id] = ticket
        return ticket

    def get_by_id(self, ticket_id: int) -> Ticket | None:
        return self._tickets.get(ticket_id)
