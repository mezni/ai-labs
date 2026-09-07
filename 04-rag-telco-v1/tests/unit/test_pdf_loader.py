from pathlib import Path

import pytest
from pypdf import PdfWriter

from src.documents import PDFLoader


def create_test_pdf(path: Path) -> None:
    """Create a small PDF used by the test."""
    writer = PdfWriter()

    writer.add_blank_page(width=612, height=792)
    writer.add_blank_page(width=612, height=792)

    with path.open("wb") as file:
        writer.write(file)


def test_pdf_loader_reads_pages(tmp_path):
    pdf_path = tmp_path / "test.pdf"

    create_test_pdf(pdf_path)

    loader = PDFLoader()

    pages = loader.load(pdf_path)

    assert len(pages) == 2
    assert pages[0].page_number == 1
    assert pages[1].page_number == 2

def test_pdf_loader_rejects_missing_file():
    loader = PDFLoader()

    with pytest.raises(FileNotFoundError):
        loader.load("does-not-exist.pdf")


def test_pdf_loader_rejects_non_pdf_file(tmp_path):
    text_file = tmp_path / "document.txt"
    text_file.write_text("hello")

    loader = PDFLoader()

    with pytest.raises(ValueError):
        loader.load(text_file)