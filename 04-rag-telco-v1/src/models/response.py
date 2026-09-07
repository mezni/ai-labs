"""Models representing RAG responses."""

from enum import StrEnum

from pydantic import BaseModel, Field


class ConfidenceLevel(StrEnum):
    """Confidence level assigned to a RAG answer."""

    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"


class Source(BaseModel):
    """Citation information returned to the user."""

    document_id: str
    document_name: str
    version: str
    page_number: int | None = None
    section_title: str | None = None


class RAGResponse(BaseModel):
    """Final response returned by the RAG application."""

    answer: str

    sources: list[Source] = Field(default_factory=list)

    confidence: ConfidenceLevel = ConfidenceLevel.LOW

    grounded: bool = False