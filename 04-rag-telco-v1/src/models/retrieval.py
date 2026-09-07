"""Models representing retrieval results."""

from uuid import UUID

from pydantic import BaseModel


class RetrievalResult(BaseModel):
    """A document chunk returned by the retriever."""

    chunk_id: UUID
    document_id: UUID
    version_id: UUID

    chunk_text: str

    score: float

    page_number: int | None = None
    section_title: str | None = None

    document_name: str
    document_version: str