from pydantic import BaseModel, Field


class Ticket(BaseModel):
    ticket_id: int
    message: str = Field(min_length=5)
    category: str
    status: str = "open"
