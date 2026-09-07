"""Document text extraction and section detection."""

import re

from pydantic import BaseModel

from src.documents.pdf_loader import PDFPage


class ExtractedPage(BaseModel):
    """Structured text extracted from one PDF page."""

    page_number: int
    text: str

    section: str | None = None
    section_title: str | None = None


class DocumentExtractor:
    """Extract structured information from loaded PDF pages."""

    SECTION_PATTERN = re.compile(
        r"^\s*(\d+(?:\.\d+)*)[\.\)]?\s+(.+?)\s*$"
    )

    def extract(self, pages: list[PDFPage]) -> list[ExtractedPage]:
        """Extract structured pages from PDF pages."""
        extracted_pages: list[ExtractedPage] = []

        current_section: str | None = None
        current_title: str | None = None

        for page in pages:
            section, title = self._find_section(page.text)

            if section:
                current_section = section
                current_title = title

            extracted_pages.append(
                ExtractedPage(
                    page_number=page.page_number,
                    text=page.text,
                    section=current_section,
                    section_title=current_title,
                )
            )

        return extracted_pages

    def _find_section(
        self,
        text: str,
    ) -> tuple[str | None, str | None]:
        """Find the first numbered section heading on a page."""
        for line in text.splitlines():
            match = self.SECTION_PATTERN.match(line)

            if match:
                return match.group(1), match.group(2).strip()

        return None, None