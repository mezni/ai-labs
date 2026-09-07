from pathlib import Path

from src.documents.policy_builder import PolicyBuilder
from src.models import DocumentType, PolicyStatus


def test_build_document():

    builder = PolicyBuilder()

    document = builder.build_document(
        document_key="roaming-policy",
        name="International Roaming Policy",
        category="roaming",
        document_type=DocumentType.POLICY,
    )

    assert document.document_key == "roaming-policy"
    assert document.name == "International Roaming Policy"
    assert document.category == "roaming"


def test_build_policy_version():

    builder = PolicyBuilder()

    document = builder.build_document(
        document_key="roaming-policy",
        name="International Roaming Policy",
        category="roaming",
        document_type=DocumentType.POLICY,
    )

    version = builder.build_version(
        document=document,
        version="2.1",
        source_file=Path("data/documents/roaming/roaming-policy.pdf"),
        source_hash="abc123",
    )

    assert version.document_id == document.id
    assert version.version == "2.1"
    assert version.status == PolicyStatus.DRAFT
    assert version.source_file_name == "roaming-policy.pdf"