"""Models representing policy documents and versions."""

from datetime import UTC, date, datetime
from enum import StrEnum
from uuid import UUID, uuid4

from pydantic import BaseModel, Field


def _utc_now() -> datetime:
    return datetime.now(UTC)


class DocumentType(StrEnum):
    """Supported policy document types."""

    POLICY = "POLICY"
    PROCEDURE = "PROCEDURE"
    FAQ = "FAQ"
    GUIDELINE = "GUIDELINE"
    COMPLIANCE = "COMPLIANCE"


class PolicyStatus(StrEnum):
    """Lifecycle status of a policy version."""

    DRAFT = "DRAFT"
    APPROVED = "APPROVED"
    ACTIVE = "ACTIVE"
    EXPIRED = "EXPIRED"
    ARCHIVED = "ARCHIVED"


class PolicyDocument(BaseModel):
    """Logical policy document independent of its versions."""

    id: UUID = Field(default_factory=uuid4)
    document_key: str
    name: str
    document_type: DocumentType
    category: str
    description: str | None = None
    owner: str | None = None
    created_at: datetime = Field(default_factory=_utc_now)
    updated_at: datetime = Field(default_factory=_utc_now)


class PolicyVersion(BaseModel):
    """A specific version of a policy document."""

    id: UUID = Field(default_factory=uuid4)
    document_id: UUID
    version: str
    status: PolicyStatus = PolicyStatus.DRAFT

    effective_date: date | None = None
    expiration_date: date | None = None

    source_file_name: str
    source_file_path: str
    source_hash: str

    approved_by: str | None = None
    approved_at: datetime | None = None

    created_at: datetime = Field(default_factory=_utc_now)
    updated_at: datetime = Field(default_factory=_utc_now)