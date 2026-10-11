import logging
import re

from support_agent.schemas import TicketInput, TicketOutput

logger = logging.getLogger(__name__)

REFUND_TERMS = ("refund", "money back")
ORDER_STATUS_TERMS = (
    "order",
    "arrived",
    "delivery",
    "shipped",
    "tracking",
    "status",
)


def classify_ticket(ticket: str) -> TicketOutput:
    validated = TicketInput(message=ticket)
    text = validated.message.lower()

    match = re.search(r"\border\s*#?\s*(\d+)\b", text)
    order_id = match.group(1) if match else None

    if any(term in text for term in REFUND_TERMS):
        category = "refund"
    elif any(term in text for term in ORDER_STATUS_TERMS):
        category = "order_status"
    else:
        category = "general"

    result = TicketOutput(
        category=category,
        order_id=order_id,
        needs_order_lookup=(category == "order_status" and order_id is not None),
    )

    logger.info(
        "ticket_classified",
        extra={
            "category": result.category,
            "has_order_id": result.order_id is not None,
        },
    )

    return result
