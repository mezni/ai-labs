import re

from support_ticket.exceptions import TicketNotFoundError
from support_ticket.models.ticket import (
    Ticket,
    TicketCategory,
)
from support_ticket.repositories.ticket_repository import (
    TicketRepository,
)


class TicketService:
    def __init__(
        self,
        repository: TicketRepository,
    ) -> None:
        self._repository = repository

    def create_ticket(self, message: str) -> Ticket:

        clean_message = message.strip()

        if len(clean_message) < 5:
            raise ValueError("Ticket message is too short")

        category = self._classify(clean_message)

        ticket = Ticket(
            ticket_id=1,
            message=clean_message,
            category=category,
        )

        return self._repository.save(ticket)

    def get_ticket(self, ticket_id: int) -> Ticket:

        ticket = self._repository.get_by_id(ticket_id)

        if ticket is None:
            raise TicketNotFoundError(f"Ticket {ticket_id} not found")

        return ticket

    @staticmethod
    def _classify(message: str) -> TicketCategory:

        if re.search(r"#\d+", message):
            return TicketCategory.ORDER_STATUS

        return TicketCategory.GENERAL
