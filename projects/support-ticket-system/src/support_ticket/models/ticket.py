from enum import StrEnum

from pydantic import BaseModel, ConfigDict, Field


class TicketStatus(StrEnum):
    OPEN = "open"
    IN_PROGRESS = "in_progress"
    CLOSED = "closed"


class TicketCategory(StrEnum):
    ORDER_STATUS = "order_status"
    REFUND = "refund"
    TECHNICAL = "technical"
    GENERAL = "general"


class Ticket(BaseModel):
    model_config = ConfigDict(
        str_strip_whitespace=True,
        validate_assignment=True,
    )

    ticket_id: int = Field(gt=0)

    message: str = Field(
        min_length=5,
        max_length=2000,
    )

    category: TicketCategory = TicketCategory.GENERAL
    status: TicketStatus = TicketStatus.OPEN
