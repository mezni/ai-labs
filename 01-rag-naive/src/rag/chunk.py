"""Character-based chunking with page tracking."""

from __future__ import annotations

from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict, Field

from rag.extract import ExtractedDocument, ExtractedPage

DEFAULT_CHUNK_SIZE = 800
DEFAULT_CHUNK_OVERLAP = 150


class Chunk(BaseModel):
    """A searchable text window with source provenance."""

    model_config = ConfigDict(frozen=True)

    chunk_id: str
    chunk_index: int = Field(ge=0)
    page_start: int = Field(gt=0)
    page_end: int = Field(gt=0)
    text: str


def make_chunk_id(document_id: str, chunk_index: int) -> str:
    """Build the deterministic logical ID for a chunk."""
    return f"{document_id}::chunk::{chunk_index}"


def chunk_document(
    document: ExtractedDocument,
    chunk_size: int = DEFAULT_CHUNK_SIZE,
    overlap: int = DEFAULT_CHUNK_OVERLAP,
) -> list[Chunk]:
    """Chunk every page of ``document``, tracking source pages per chunk."""
    return chunk_pages(document.pages, document.document_id, chunk_size, overlap)


def chunk_pages(
    pages: Iterable[ExtractedPage],
    document_id: str,
    chunk_size: int = DEFAULT_CHUNK_SIZE,
    overlap: int = DEFAULT_CHUNK_OVERLAP,
) -> list[Chunk]:
    """Split ``pages`` into character windows of ``chunk_size``.

    Windows advance by ``chunk_size - overlap`` characters. Each chunk records
    the first and last page it spans, and gets a deterministic
    ``<document_id>::chunk::<index>`` ID.
    """
    if chunk_size < 1:
        raise ValueError("chunk_size must be >= 1")
    if overlap < 0:
        raise ValueError("overlap must be >= 0")
    step = chunk_size - overlap
    if step < 1:
        raise ValueError("overlap must be smaller than chunk_size")

    text, spans = _build_paged_text(pages)

    chunks: list[Chunk] = []
    start = 0
    index = 0
    length = len(text)
    while start < length:
        end = min(start + chunk_size, length)
        content = text[start:end].strip()
        if content:
            chunks.append(
                Chunk(
                    chunk_id=make_chunk_id(document_id, index),
                    chunk_index=index,
                    page_start=_page_at(spans, start),
                    page_end=_page_at(spans, end - 1),
                    text=content,
                )
            )
            index += 1
        start += step
    return chunks


def _build_paged_text(pages: Iterable[ExtractedPage]) -> tuple[str, list[tuple[int, int, int]]]:
    """Concatenate page texts, returning the joined text and per-page spans."""
    parts: list[str] = []
    spans: list[tuple[int, int, int]] = []
    offset = 0
    for index, page in enumerate(pages):
        if index > 0:
            parts.append("\n\n")
            offset += 2
        parts.append(page.text)
        spans.append((offset, offset + len(page.text), page.page_number))
        offset += len(page.text)
    return "".join(parts), spans


def _page_at(spans: list[tuple[int, int, int]], offset: int) -> int:
    """Return the page containing ``offset`` in the joined text."""
    for _start, end, page in spans:
        if offset < end:
            return page
    return spans[-1][2]
