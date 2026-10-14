from typing import Literal

from pydantic import BaseModel, ConfigDict


class Block(BaseModel):
    """A unit of parsed text with the lineage needed for chunking and citations."""

    model_config = ConfigDict(frozen=True)

    kind: Literal["heading", "paragraph"]
    text: str
    level: int | None = None  # heading level (1-6), headings only
    page: int | None = None  # 1-based page number, PDFs only
