"""Text cleaning and normalization."""

from __future__ import annotations

import re
from collections.abc import Iterable

from rag.extract import ExtractedPage

_HORIZONTAL_WHITESPACE = re.compile(r"[ \t]+")


def clean_text(text: str) -> str:
    """Normalize extracted text.

    Converts ``\\r\\n`` and ``\\r`` to ``\\n``, removes null characters, collapses
    horizontal whitespace runs and single line breaks into single spaces,
    preserves blank-line paragraph boundaries, and trims leading/trailing
    whitespace.
    """
    normalized = text.replace("\r\n", "\n").replace("\r", "\n")
    normalized = normalized.replace("\x00", "")

    lines = [_HORIZONTAL_WHITESPACE.sub(" ", line).strip() for line in normalized.split("\n")]

    paragraphs: list[str] = []
    current: list[str] = []
    for line in lines:
        if line == "":
            paragraphs.append(" ".join(current))
            current = []
        else:
            current.append(line)
    paragraphs.append(" ".join(current))

    return "\n\n".join(paragraph for paragraph in paragraphs if paragraph)


def clean_pages(pages: Iterable[ExtractedPage]) -> list[ExtractedPage]:
    """Clean every page in ``pages`` while preserving page numbering."""
    return [
        ExtractedPage(page_number=page.page_number, text=clean_text(page.text)) for page in pages
    ]
