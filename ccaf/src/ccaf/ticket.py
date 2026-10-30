from dataclasses import dataclass


@dataclass
class Ticket:
    ticket_id: str
    message: str


@dataclass
class Classification:
    category: str
    confidence: float


def prepare_classification_request(ticket: Ticket) -> dict:
    return {
        "ticket_id": ticket.ticket_id,
        "message": ticket.message,
        "task": "Classify this customer support ticket",
        "categories": [
            "billing",
            "technical_support",
            "order_status",
            "refund",
            "other",
        ],
    }


ticket = Ticket(
    ticket_id="TICKET-001",
    message="My internet connection has stopped working.",
)

request = prepare_classification_request(ticket)


print(request)
