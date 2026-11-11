import re

from support_ticket.models.ticket import Ticket
from support_ticket.repositories.ticket_repository import TicketRepository


class TicketService:
    def __init__(self, repository: TicketRepository):
        self.repository = repository

    def create_ticket(self, message: str) -> Ticket:
        order_match = re.search(r"#(\d+)", message)

        category = "order_status" if order_match else "general"

        ticket = Ticket(
            ticket_id=1,
            message=message,
            category=category,
        )

        return self.repository.save(ticket)
