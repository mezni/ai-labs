"""Tests for KnowledgeBase._chunk_text."""
import pytest

LONG_TEXT = " ".join(["policy"] * 40)


def test_chunk_uses_instance_defaults(kb_factory):
    kb = kb_factory(chunk_size=50, overlap=10)

    chunks = kb._chunk_text(LONG_TEXT, document_id="leave", source="hr/leave.md")

    assert len(chunks) > 1
    assert all(chunk["content"] for chunk in chunks)
    assert all(len(chunk["content"]) <= 50 for chunk in chunks)
    total = sum(len(chunk["content"]) for chunk in chunks)
    assert total <= len(LONG_TEXT) + 10 * len(chunks)


def test_chunk_explicit_params_override_instance(kb_factory):
    kb = kb_factory(chunk_size=500, overlap=50)

    chunks = kb._chunk_text(
        LONG_TEXT,
        document_id="leave",
        source="hr/leave.md",
        chunk_size=30,
        overlap=0,
    )

    assert len(chunks) > 1
    assert all(len(chunk["content"]) <= 30 for chunk in chunks)


def test_chunk_metadata_and_id(kb_factory):
    kb = kb_factory()

    chunks = kb._chunk_text("Some content", document_id="leave", source="hr/leave.md")

    assert len(chunks) == 1
    assert chunks[0] == {
        "chunk_id": "leave_chunk_001",
        "content": "Some content",
        "document_id": "leave",
        "source": "hr/leave.md",
        "category": "hr",
    }


def test_chunk_multiple_chunks_are_numbered(kb_factory):
    kb = kb_factory(chunk_size=30, overlap=0)

    chunks = kb._chunk_text(LONG_TEXT, document_id="backup", source="it/backup.md")

    assert len(chunks) == 10
    assert [chunk["chunk_id"] for chunk in chunks] == [
        f"backup_chunk_{i:03d}" for i in range(1, 11)
    ]
    assert all(chunk["category"] == "it" for chunk in chunks)


@pytest.mark.parametrize(
    ("source", "category"),
    [
        ("hr/leave.md", "hr"),
        ("it/deep/access.md", "it"),
        ("overview.md", "policies"),
    ],
)
def test_derive_category(kb_factory, source, category):
    kb = kb_factory()

    assert kb._derive_category(source) == category