"""Document chunking."""

from src.documents.extractor import ExtractedPage
from src.models import DocumentChunk


class ChunkingService:
    """Split extracted document pages into searchable chunks."""

    def __init__(
        self,
        chunk_size: int = 500,
        chunk_overlap: int = 50,
    ) -> None:
        if chunk_size <= 0:
            raise ValueError("chunk_size must be greater than zero")

        if chunk_overlap < 0:
            raise ValueError("chunk_overlap cannot be negative")

        if chunk_overlap >= chunk_size:
            raise ValueError(
                "chunk_overlap must be smaller than chunk_size"
            )

        self.chunk_size = chunk_size
        self.chunk_overlap = chunk_overlap

    def chunk_pages(
        self,
        pages: list[ExtractedPage],
        document_id,
        version_id,
    ) -> list[DocumentChunk]:
        """Create chunks while preserving page and section metadata."""

        chunks: list[DocumentChunk] = []
        chunk_index = 0

        for page in pages:
            page_chunks = self._split_text(page.text)

            for text in page_chunks:
                chunks.append(
                    DocumentChunk(
                        document_id=document_id,
                        version_id=version_id,
                        chunk_index=chunk_index,
                        chunk_text=text,
                        page_number=page.page_number,
                        section=page.section,
                        section_title=page.section_title,
                        metadata={
                            "page_number": str(page.page_number),
                            "section": page.section or "",
                            "section_title": page.section_title or "",
                        },
                    )
                )

                chunk_index += 1

        return chunks

    def _split_text(self, text: str) -> list[str]:
        """Split text into overlapping character-based chunks."""

        text = text.strip()

        if not text:
            return []

        chunks: list[str] = []

        start = 0
        text_length = len(text)

        while start < text_length:
            end = min(start + self.chunk_size, text_length)

            chunk = text[start:end].strip()

            if chunk:
                chunks.append(chunk)

            if end >= text_length:
                break

            start = end - self.chunk_overlap

        return chunks