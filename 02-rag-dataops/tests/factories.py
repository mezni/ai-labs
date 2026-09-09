"""Test factories: small PDF/DOCX/HTML/TXT builders (mirrors 01-rag-naive)."""

from __future__ import annotations

from pathlib import Path

from reportlab.pdfgen import canvas as pdf_canvas


def build_pdf(path: Path, text: str) -> None:
    """Write a single-page PDF whose extracted text contains ``text`` lines."""
    path.parent.mkdir(parents=True, exist_ok=True)
    canvas = pdf_canvas.Canvas(str(path))
    y = 760
    for line in text.splitlines():
        canvas.drawString(72, y, line)
        y -= 18
    canvas.save()


def build_docx(path: Path, paragraphs: list[str]) -> None:
    from docx import Document

    path.parent.mkdir(parents=True, exist_ok=True)
    document = Document()
    for paragraph in paragraphs:
        document.add_paragraph(paragraph)
    document.save(str(path))


def build_html(path: Path, headings: list[str], paragraphs: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    html_parts = ["<!doctype html><html><head><title>test</title></head><body>"]
    for heading in headings:
        html_parts.append(f"<h1>{heading}</h1>")
    for paragraph in paragraphs:
        html_parts.append(f"<p>{paragraph}</p>")
    html_parts.extend(["<script>console.log('ignore me')</script>", "</body></html>"])
    path.write_text("".join(html_parts), encoding="utf-8")


def build_txt(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def build_sample_database(path: Path, rows: list[tuple[int, str, str]]) -> None:
    import sqlite3

    path.parent.mkdir(parents=True, exist_ok=True)
    with sqlite3.connect(path) as connection:
        connection.execute(
            "CREATE TABLE IF NOT EXISTS documents ("
            "id INTEGER PRIMARY KEY, title TEXT, body TEXT)"
        )
        connection.executemany(
            "INSERT OR REPLACE INTO documents (id, title, body) VALUES (?, ?, ?)",
            rows,
        )