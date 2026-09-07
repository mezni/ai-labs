from src.models import (
    DocumentType,
    PolicyDocument,
    PolicyStatus,
    PolicyVersion,
)


def test_policy_document_creation():
    document = PolicyDocument(
        document_key="roaming-policy",
        name="International Roaming Policy",
        document_type=DocumentType.POLICY,
        category="roaming",
    )

    assert document.document_key == "roaming-policy"
    assert document.document_type == DocumentType.POLICY
    assert document.category == "roaming"


def test_policy_version_defaults_to_draft():
    document = PolicyDocument(
        document_key="roaming-policy",
        name="International Roaming Policy",
        document_type=DocumentType.POLICY,
        category="roaming",
    )

    version = PolicyVersion(
        document_id=document.id,
        version="1.0",
        source_file_name="roaming-policy.pdf",
        source_file_path="data/documents/roaming/roaming-policy.pdf",
        source_hash="abc123",
    )

    assert version.status == PolicyStatus.DRAFT