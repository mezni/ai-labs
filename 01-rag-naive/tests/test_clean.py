"""Tests for ``rag.clean`` text normalization."""

import pytest

from rag.clean import clean_pages, clean_text
from rag.extract import ExtractedPage


def test_normalizes_crlf_and_cr_line_endings():
    assert clean_text("line1\r\nline2\rline3") == "line1 line2 line3"


def test_removes_null_characters():
    assert clean_text("a\x00b\x00c") == "abc"


def test_collapses_excess_whitespace():
    assert clean_text("Hello       World") == "Hello World"


def test_collapses_tabs():
    assert clean_text("a\t\tb\tc") == "a b c"


def test_merges_single_line_breaks_within_paragraph():
    assert clean_text("Customer\nrefund requests") == "Customer refund requests"


def test_preserves_paragraph_boundaries():
    assert clean_text("First paragraph\n\nSecond paragraph") == (
        "First paragraph\n\nSecond paragraph"
    )


def test_collapses_repeated_blank_lines_to_single_break():
    assert clean_text("A\n\n\n\nB") == "A\n\nB"


def test_trims_leading_and_trailing_whitespace():
    assert clean_text("  padded  text  ") == "padded text"


def test_removes_leading_and_trailing_blank_lines():
    assert clean_text("\n\nLeading blank lines\n\n") == "Leading blank lines"


@pytest.mark.parametrize("text", ["", "   ", "\n\n\n"])
def test_empty_and_whitespace_only_text(text):
    assert clean_text(text) == ""


def test_clean_pages_preserves_page_numbering():
    pages = [
        ExtractedPage(page_number=1, text="first  page"),
        ExtractedPage(page_number=2, text="second\r\npage"),
    ]
    cleaned = clean_pages(pages)

    assert [page.page_number for page in cleaned] == [1, 2]
    assert [page.text for page in cleaned] == ["first page", "second page"]
