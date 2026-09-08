"""Tests for ``rag.chunk`` character-based chunking."""

import pytest
from pydantic import ValidationError

from rag.chunk import Chunk, chunk_document, chunk_pages, make_chunk_id
from rag.extract import ExtractedDocument, ExtractedPage


def _document(*page_texts: str, document_id: str = "billing.pdf") -> ExtractedDocument:
    pages = [
        ExtractedPage(page_number=index, text=text)
        for index, text in enumerate(page_texts, start=1)
    ]
    return ExtractedDocument(document_id=document_id, pages=pages)


def _chunk(doc: ExtractedDocument, chunk_size: int, overlap: int) -> list[Chunk]:
    return chunk_document(doc, chunk_size=chunk_size, overlap=overlap)


def test_make_chunk_id_follows_design_format():
    assert make_chunk_id("billing.pdf", 0) == "billing.pdf::chunk::0"
    assert make_chunk_id("roaming_policy.pdf", 7) == "roaming_policy.pdf::chunk::7"


def test_short_document_produces_single_chunk():
    doc = _document("Aurora Mobile Billing Policy. Refunds within 30 days.")

    chunks = _chunk(doc, chunk_size=800, overlap=150)

    assert len(chunks) == 1
    chunk = chunks[0]
    assert chunk.chunk_index == 0
    assert chunk.chunk_id == "billing.pdf::chunk::0"
    assert chunk.page_start == 1
    assert chunk.page_end == 1


def test_empty_document_produces_no_chunks():
    doc = _document("")

    assert _chunk(doc, chunk_size=800, overlap=150) == []


def test_whitespace_only_document_produces_no_chunks():
    doc = _document("   \n\n  ")

    assert _chunk(doc, chunk_size=800, overlap=150) == []


def test_long_single_page_splits_into_ordered_chunks():
    doc = _document("word " * 300)

    chunks = _chunk(doc, chunk_size=100, overlap=10)

    assert len(chunks) > 1
    assert all(chunk.text for chunk in chunks)
    assert all(chunk.page_start == 1 and chunk.page_end == 1 for chunk in chunks)
    assert [chunk.chunk_index for chunk in chunks] == list(range(len(chunks)))
    assert [chunk.chunk_id for chunk in chunks] == [
        make_chunk_id("billing.pdf", index) for index in range(len(chunks))
    ]


def test_no_chunk_exceeds_chunk_size():
    doc = _document(" ".join(f"page one paragraph {i}" for i in range(200)))

    chunks = _chunk(doc, chunk_size=100, overlap=10)

    assert chunks
    assert all(len(chunk.text) <= 100 for chunk in chunks)


def test_consecutive_chunks_overlap():
    doc = _document("A" * 100, "B" * 100)

    chunks = _chunk(doc, chunk_size=150, overlap=20)

    assert len(chunks) == 2
    assert chunks[1].text.startswith(chunks[0].text[-20:])


def test_chunk_across_page_boundary_tracks_pages():
    doc = _document("A" * 100, "B" * 100)

    chunks = _chunk(doc, chunk_size=150, overlap=20)

    assert len(chunks) == 2
    assert (chunks[0].page_start, chunks[0].page_end) == (1, 2)
    assert (chunks[1].page_start, chunks[1].page_end) == (2, 2)


def test_chunks_spanning_multiple_pages_track_start_and_end_pages():
    doc = _document("X" * 50, "Y" * 50, "Z" * 50)

    chunks = _chunk(doc, chunk_size=80, overlap=20)

    assert len(chunks) == 3
    assert [(chunk.page_start, chunk.page_end) for chunk in chunks] == [
        (1, 2),
        (2, 3),
        (3, 3),
    ]
    assert chunks[1].text.startswith(chunks[0].text[-20:])
    assert chunks[2].text.startswith(chunks[1].text[-20:])


def test_chunk_document_uses_document_id():
    doc = _document("some text", document_id="roaming_policy.pdf")

    chunks = chunk_document(doc, chunk_size=800, overlap=150)

    assert chunks[0].chunk_id.startswith("roaming_policy.pdf::chunk::")


def test_chunk_pages_accepts_plain_iterable():
    pages = [ExtractedPage(page_number=1, text="hello world")]

    chunks = chunk_pages(pages, document_id="doc.pdf", chunk_size=800, overlap=150)

    assert len(chunks) == 1
    assert chunks[0].text == "hello world"


def test_empty_pages_list_yields_no_chunks():
    assert chunk_pages([], document_id="empty.pdf") == []


@pytest.mark.parametrize("chunk_size", [0, -1])
def test_rejects_non_positive_chunk_size(chunk_size):
    doc = _document("text")

    with pytest.raises(ValueError, match="chunk_size"):
        _chunk(doc, chunk_size=chunk_size, overlap=0)


def test_rejects_negative_overlap():
    doc = _document("text")

    with pytest.raises(ValueError, match="overlap"):
        _chunk(doc, chunk_size=10, overlap=-1)


def test_rejects_overlap_geq_chunk_size():
    doc = _document("text")

    with pytest.raises(ValueError, match="overlap"):
        _chunk(doc, chunk_size=10, overlap=10)


@pytest.mark.parametrize(
    "invalid",
    [
        {"chunk_index": -1, "page_start": 1, "page_end": 1},
        {"chunk_index": 0, "page_start": 0, "page_end": 1},
        {"chunk_index": 0, "page_start": 1, "page_end": 0},
    ],
)
def test_chunk_model_validates_fields(invalid):
    fields = {"chunk_id": "doc::chunk::0", "text": "content"}

    with pytest.raises(ValidationError):
        Chunk(**fields, **invalid)


def test_chunk_model_dump_matches_design_shape():
    chunk = Chunk(
        chunk_id="billing.pdf::chunk::0",
        chunk_index=0,
        page_start=1,
        page_end=1,
        text="content",
    )

    data = chunk.model_dump(mode="json")

    assert set(data) == {"chunk_id", "chunk_index", "page_start", "page_end", "text"}
    assert data["chunk_id"] == "billing.pdf::chunk::0"
    assert data["chunk_index"] == 0
    assert data["page_start"] == 1 and data["page_end"] == 1
    assert data["text"] == "content"
