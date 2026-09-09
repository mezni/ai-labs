from __future__ import annotations

import pytest

from src.ingestion.parser import (
    ParseError,
    parse_database_row,
    parse_docx,
    parse_file,
    parse_html,
    parse_pdf,
    parse_text_file,
)


def test_parse_pdf(tmp_path):
    from tests.factories import build_pdf

    path = tmp_path / "refund-policy.pdf"
    build_pdf(path, "Line one\nLine two")
    document = parse_pdf(path, "refund-policy.pdf")
    assert document.format == "pdf"
    assert "Line one" in document.text
    assert "Line two" in document.text
    assert document.metadata["pages"] == 1


def test_parse_pdf_failure(tmp_path):
    path = tmp_path / "broken.pdf"
    path.write_bytes(b"not a real pdf")
    with pytest.raises(ParseError):
        parse_pdf(path, "broken.pdf")


def test_parse_docx(tmp_path):
    from tests.factories import build_docx

    path = tmp_path / "handbook.docx"
    build_docx(path, ["First paragraph", "Second paragraph"])
    document = parse_docx(path, "handbook.docx")
    assert document.format == "docx"
    assert "First paragraph" in document.text
    assert "Second paragraph" in document.text


def test_parse_html_strips_scripts(tmp_path):
    from tests.factories import build_html

    path = tmp_path / "faq.html"
    build_html(path, ["Reimbursements"], ["Maximum 500 EUR."])
    document = parse_html(
        path.read_text(encoding="utf-8"), "faq.html", source="web", url="http://x"
    )
    assert document.format == "html"
    assert "Maximum 500 EUR." in document.text
    assert "ignore me" not in document.text
    assert document.metadata["url"] == "http://x"
    assert document.metadata["headings"] == ["Reimbursements"]


def test_parse_text_file(tmp_path):
    from tests.factories import build_txt

    path = tmp_path / "notes.txt"
    build_txt(path, "Just some notes.")
    document = parse_text_file(path, "notes.txt")
    assert document.format == "txt"
    assert document.text == "Just some notes."


def test_parse_database_row_projection():
    document = parse_database_row(
        table="billing",
        primary_key="id",
        row={"id": 42, "title": "Invoice", "amount": 10.0},
        projection_columns=["title", "amount"],
    )
    assert document.document_id == "billing/42"
    assert document.format == "database_row"
    assert "billing" in document.text
    assert "title: Invoice" in document.text
    assert document.metadata["table"] == "billing"
    assert document.metadata["row"]["amount"] == 10.0


def test_parse_file_routes_by_extension(tmp_path):
    from tests.factories import build_txt

    path = tmp_path / "a.txt"
    build_txt(path, "hi")
    document = parse_file(path, "a.txt")
    assert document.format == "txt"


def test_parse_file_unknown_extension(tmp_path):
    path = tmp_path / "a.xyz"
    path.write_text("hi")
    with pytest.raises(ParseError):
        parse_file(path, "a.xyz")