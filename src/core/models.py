from typing import Literal

from pydantic import BaseModel, ConfigDict


class Block(BaseModel):
    """A unit of parsed text with the lineage needed for chunking and citations."""

    model_config = ConfigDict(frozen=True)

    kind: Literal["heading", "paragraph"]
    text: str
    level: int | None = None  # heading level (1-6), headings only
    page: int | None = None  # 1-based page number, PDFs only


class Chunk(BaseModel):
    """A retrievable piece of a document, with full lineage for filtering and citations."""

    model_config = ConfigDict(frozen=True)

    chunk_id: str  # deterministic: <doc_id>-v<version>-<ordinal>
    doc_id: str
    version: int
    domain: str
    ordinal: int  # position of the chunk inside its document
    text: str
    heading_path: tuple[str, ...]  # e.g. ("AW-ROA-002 Roaming Policy (v2)", "2. Zones")
    clause: str | None = None  # clause number of the first clause, e.g. "2.2"
    page: int | None = None  # page of the first block, PDFs only
    token_count: int

    def text_with_context(self) -> str:
        """Text to embed: the heading path gives the chunk its context."""
        if not self.heading_path:
            return self.text
        return f"{' > '.join(self.heading_path)}\n{self.text}"
