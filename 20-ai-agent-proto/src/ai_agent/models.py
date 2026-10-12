from enum import Enum
from pydantic import BaseModel, ConfigDict, Field, field_validator


class TicketCategory(str, Enum):
    REFUND_DISPUTE = "refund_dispute"
    ORDER_STATUS = "order_status"
    TECHNICAL_SUPPORT = "technical_support"
    GENERAL_INQUIRY = "general_inquiry"


class RawTicketInput(BaseModel):
    """Data contract for incoming raw ticket payloads."""

    model_config = ConfigDict(extra="ignore", str_strip_whitespace=True)

    text: str = Field(..., description="Raw customer ticket text")
    user_id: str | None = Field(default=None, description="Sender metadata ID")

    @field_validator("text")
    @classmethod
    def validate_non_empty(cls, value: str) -> str:
        if not value.strip():
            raise ValueError("Policy Violation: Ticket text cannot be empty or pure whitespace.")
        return value


class OperationalMetadata(BaseModel):
    """Operational telemetry — excludes raw customer text to preserve privacy."""

    model_config = ConfigDict(extra="forbid")

    text_length: int
    word_count: int
    has_order_id_pattern: bool


class TicketClassification(BaseModel):
    """Strict execution output contract."""

    model_config = ConfigDict(extra="forbid")

    category: TicketCategory
    confidence: float = Field(..., ge=0.0, le=1.0)
    order_id: str | None = None
    needs_order_lookup: bool = False
    needs_human_review: bool = False
    assumptions: list[str] = Field(default_factory=list)
    metadata: OperationalMetadata


class LLMTicketClassification(BaseModel):
    """Schema enforced on the LLM output via structured function calling."""

    model_config = ConfigDict(extra="forbid")

    category: TicketCategory = Field(
        ..., description="The primary semantic category of the ticket."
    )
    confidence: float = Field(
        ..., ge=0.0, le=1.0, description="Confidence score between 0.0 and 1.0."
    )
    order_id: str | None = Field(
        default=None,
        description="Extracted order ID (numeric digits only), if mentioned.",
    )
    reasoning: str = Field(
        ..., description="Brief 1-sentence explanation of why this category was chosen."
    )