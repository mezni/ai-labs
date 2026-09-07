"""Models representing document chunks used for retrieval."""

from uuid import UUID, uuid4

from pydantic import BaseModel, Field


class DocumentChunk(BaseModel):
    """A searchable chunk extracted from a policy version."""

    id: UUID = Field(default_factory=uuid4)

    document_id: UUID
    version_id: UUID

    chunk_index: int
    chunk_text: str

    page_number: int | None = None
    section: str | None = None
    section_title: str | None = None

    token_count: int | None = None

    metadata: dict[str, str] = Field(default_factory=dict)