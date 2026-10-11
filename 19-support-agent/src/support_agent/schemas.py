from typing import Literal

from pydantic import BaseModel, Field, StrictBool, field_validator


class TicketInput(BaseModel):
    message: str = Field(min_length=1, max_length=5000)

    @field_validator("message", mode="before")
    @classmethod
    def strip_message(cls, v):
        if isinstance(v, str):
            return v.strip()
        return v


class TicketOutput(BaseModel):
    category: Literal["order_status", "refund", "general"]
    order_id: str | None
    needs_order_lookup: StrictBool
