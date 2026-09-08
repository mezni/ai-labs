"""Tests for ``rag.extract`` PDF discovery and page-aware extraction."""

from pathlib import Path

import pytest
from pydantic import ValidationError

from rag.extract import (
    ExtractedDocument,
    ExtractedPage,
    ExtractionError,
    discover_pdfs,
    extract_pdf,
)
from tests.pdf_factory import build_pdf


def _write_pdf(directory: Path, name: str, pages: list[str]) -> Path:
    path = directory / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(build_pdf(pages))
    return path


def test_discover_pdfs_returns_sorted_relative_paths(tmp_path):
    _write_pdf(tmp_path, "b.pdf", ["B"])
    _write_pdf(tmp_path / "sub", "a.pdf", ["A"])
    _write_pdf(tmp_path / "sub" / "nested", "c.PDF", ["C"])
    _write_pdf(tmp_path, "notes.txt", ["not a pdf"])

    assert discover_pdfs(tmp_path) == ["b.pdf", "sub/a.pdf", "sub/nested/c.PDF"]


def test_discover_pdfs_empty_directory(tmp_path):
    assert discover_pdfs(tmp_path) == []


def test_discover_pdfs_missing_directory(tmp_path):
    assert discover_pdfs(tmp_path / "does-not-exist") == []


@pytest.fixture()
def two_page_document(tmp_path) -> Path:
    pages = [
        "Aurora Mobile Billing Policy. Customers may request a refund within 30 days "
        "of the original billing date.",
        "Refund requests are accepted for unused services only. Late payments may be "
        "subject to a fee of 5 percent of the outstanding balance.",
    ]
    return _write_pdf(tmp_path, "billing.pdf", pages)


def test_extract_pdf_preserves_page_boundaries(two_page_document: Path):
    doc = extract_pdf(two_page_document, document_id="billing.pdf")

    assert isinstance(doc, ExtractedDocument)
    assert doc.document_id == "billing.pdf"
    assert isinstance(doc.pages, list)
    assert len(doc.pages) == 2
    assert doc.pages == [
        ExtractedPage(page_number=1, text=doc.pages[0].text),
        ExtractedPage(page_number=2, text=doc.pages[1].text),
    ]


def test_extract_pdf_extracts_page_text(two_page_document: Path):
    pages = [
        "Aurora Mobile Billing Policy. Customers may request a refund within 30 days "
        "of the original billing date.",
        "Refund requests are accepted for unused services only. Late payments may be "
        "subject to a fee of 5 percent of the outstanding balance.",
    ]
    doc = extract_pdf(two_page_document, document_id="billing.pdf")

    normalized = [" ".join(page.text.split()) for page in doc.pages]
    assert normalized == pages


def test_extract_pdf_defaults_document_id_to_filename(two_page_document: Path):
    assert extract_pdf(two_page_document).document_id == "billing.pdf"


def test_extract_document_model_dump_matches_design_shape(two_page_document: Path):
    doc = extract_pdf(two_page_document, document_id="billing.pdf")
    data = doc.model_dump(mode="json")

    assert set(data) == {"document_id", "pages"}
    assert data["document_id"] == "billing.pdf"
    assert len(data["pages"]) == 2
    page = data["pages"][0]
    assert set(page) == {"page_number", "text"}
    assert page["page_number"] == 1
    assert isinstance(page["text"], str) and page["text"]


def test_extract_pdf_raises_on_invalid_pdf(tmp_path):
    broken = tmp_path / "broken.pdf"
    broken.write_bytes(b"this is not a pdf")

    with pytest.raises(ExtractionError):
        extract_pdf(broken)


def test_extracted_page_requires_positive_page_number():
    with pytest.raises(ValidationError):
        ExtractedPage(page_number=0, text="irrelevant")
