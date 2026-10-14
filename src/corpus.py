import json
from datetime import date
from pathlib import Path

from pydantic import BaseModel

from chunking import chunk_blocks
from cleaning import clean
from core.models import Chunk
from parsing import parse


class ManifestEntry(BaseModel):
    doc_id: str
    version: int
    title: str
    domain: str
    path: str
    content_type: str
    plan_types: list[str]
    regions: list[str]
    effective_from: date
    effective_to: date | None
    status: str


def load_manifest(data_dir: Path) -> list[ManifestEntry]:
    raw = json.loads((data_dir / "manifest.json").read_text(encoding="utf-8"))
    return [ManifestEntry.model_validate(entry) for entry in raw]


def chunk_document(
    data_dir: Path, entry: ManifestEntry, *, max_tokens: int = 200, min_tokens: int = 20
) -> list[Chunk]:
    """Read one manifest document and run it through parse, clean and chunk."""
    data = (data_dir / entry.path).read_bytes()
    blocks = clean(parse(data, entry.content_type))
    return chunk_blocks(
        blocks,
        doc_id=entry.doc_id,
        version=entry.version,
        domain=entry.domain,
        max_tokens=max_tokens,
        min_tokens=min_tokens,
    )
