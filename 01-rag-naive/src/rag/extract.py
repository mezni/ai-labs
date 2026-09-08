"""PDF discovery and page-aware text extraction."""

from __future__ import annotations

import logging
from pathlib import Path

from pydantic import BaseModel, ConfigDict, Field
from pypdf import PdfReader

logger = logging.getLogger(__name__)


class ExtractionError(RuntimeError):
    """Raised when a PDF cannot be read or its text cannot be extracted."""


class ExtractedPage(BaseModel):
    """Text extracted from a single PDF page."""

    model_config = ConfigDict(frozen=True)

    page_number: int = Field(gt=0)
    text: str


class ExtractedDocument(BaseModel):
    """Page-level text for a single PDF document."""

    model_config = ConfigDict(frozen=True)

    document_id: str
    pages: list[ExtractedPage]


def discover_pdfs(data_dir: Path) -> list[str]:
    """Return sorted, data-dir-relative paths of every PDF under ``data_dir``."""
    if not data_dir.is_dir():
        return []
    pdfs = [
        path for path in data_dir.rglob("*") if path.is_file() and path.suffix.lower() == ".pdf"
    ]
    return sorted(path.relative_to(data_dir).as_posix() for path in pdfs)


def extract_pdf(pdf_path: Path, document_id: str | None = None) -> ExtractedDocument:
    """Extract page-level text from a PDF, preserving page boundaries.

    ``document_id`` defaults to the file name. Raises :class:`ExtractionError`
    if the PDF cannot be read or a page cannot be extracted.
    """
    resolved_document_id = pdf_path.name if document_id is None else document_id
    try:
        reader = PdfReader(str(pdf_path))
        pages = [
            ExtractedPage(page_number=index, text=page.extract_text() or "")
            for index, page in enumerate(reader.pages, start=1)
        ]
        return ExtractedDocument(document_id=resolved_document_id, pages=pages)
    except ExtractionError:
        raise
    except Exception as exc:
        logger.error("PDF extraction failed for %s: %s", pdf_path, exc)
        raise ExtractionError(f"failed to extract {pdf_path}: {exc}") from exc
