"""PDF loading."""

from pathlib import Path

from pydantic import BaseModel, Field
from pypdf import PdfReader


class PDFPage(BaseModel):
    """Represents one page loaded from a PDF."""

    page_number: int
    text: str = ""
    metadata: dict[str, str] = Field(default_factory=dict)


class PDFLoader:
    """Load text from PDF files while preserving page boundaries."""

    def load(self, file_path: str | Path) -> list[PDFPage]:
        """Load a PDF and return one PDFPage object per page."""
        path = Path(file_path)

        if not path.exists():
            raise FileNotFoundError(f"PDF file not found: {path}")

        if path.suffix.lower() != ".pdf":
            raise ValueError(f"Expected a PDF file, got: {path.suffix}")

        reader = PdfReader(path)

        pages: list[PDFPage] = []

        for page_number, page in enumerate(reader.pages, start=1):
            text = page.extract_text() or ""

            pages.append(
                PDFPage(
                    page_number=page_number,
                    text=text,
                    metadata={
                        "source_file": path.name,
                    },
                )
            )

        return pages