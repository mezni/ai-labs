"""Build policy models from extracted document information."""

from datetime import date
from pathlib import Path

from src.models import (
    DocumentType,
    PolicyDocument,
    PolicyStatus,
    PolicyVersion,
)


class PolicyBuilder:
    """Create policy domain models from document metadata."""

    def build_document(
        self,
        *,
        document_key: str,
        name: str,
        category: str,
        document_type: DocumentType,
        description: str | None = None,
        owner: str | None = None,
    ) -> PolicyDocument:
        """Create a logical policy document."""
        return PolicyDocument(
            document_key=document_key,
            name=name,
            document_type=document_type,
            category=category,
            description=description,
            owner=owner,
        )

    def build_version(
        self,
        *,
        document: PolicyDocument,
        version: str,
        source_file: str | Path,
        source_hash: str,
        effective_date: date | None = None,
    ) -> PolicyVersion:
        """Create a policy version."""
        source_file = Path(source_file)

        return PolicyVersion(
            document_id=document.id,
            version=version,
            status=PolicyStatus.DRAFT,
            effective_date=effective_date,
            source_file_name=source_file.name,
            source_file_path=str(source_file),
            source_hash=source_hash,
        )