"""Document hashing, change detection, and document-state persistence."""

from __future__ import annotations

import hashlib
import json
import logging
import os
from collections.abc import Mapping
from enum import StrEnum
from pathlib import Path

from pydantic import BaseModel, ConfigDict, Field

from rag.extract import discover_pdfs

logger = logging.getLogger(__name__)


class Change(StrEnum):
    """Status of a document relative to the previously indexed state."""

    NEW = "NEW"
    UNCHANGED = "UNCHANGED"
    CHANGED = "CHANGED"
    DELETED = "DELETED"


class DocumentState(BaseModel):
    """Record of the indexed version of a single document."""

    model_config = ConfigDict(frozen=True)

    hash: str
    vector_ids: list[int]
    chunk_count: int = Field(ge=0)


def sha256_file(path: Path) -> str:
    """Return the lowercase SHA-256 hex digest of a file's contents."""
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(65536), b""):
            digest.update(block)
    return digest.hexdigest()


def hash_documents(data_dir: Path) -> dict[str, str]:
    """Map every PDF's data-dir-relative path to its SHA-256 hash."""
    return {
        relative_path: sha256_file(data_dir / relative_path)
        for relative_path in discover_pdfs(data_dir)
    }


def detect_changes(
    current_hashes: Mapping[str, str],
    previous_state: Mapping[str, DocumentState],
) -> dict[str, Change]:
    """Classify every document as NEW, UNCHANGED, CHANGED, or DELETED."""
    changes: dict[str, Change] = {}
    for relative_path, current_hash in current_hashes.items():
        previous = previous_state.get(relative_path)
        if previous is None:
            changes[relative_path] = Change.NEW
        elif previous.hash == current_hash:
            changes[relative_path] = Change.UNCHANGED
        else:
            changes[relative_path] = Change.CHANGED
    for relative_path in set(previous_state) - set(current_hashes):
        changes[relative_path] = Change.DELETED
    return changes


def atomic_write_json(path: Path, payload: object) -> None:
    """Write ``payload`` as pretty JSON to ``path`` atomically (``.tmp`` + replace)."""
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp_path = path.with_name(f"{path.name}.tmp")
    with tmp_path.open("w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2)
        handle.write("\n")
    os.replace(tmp_path, path)


class DocumentStateStore:
    """Persist ``{document_id: DocumentState}`` as JSON."""

    def __init__(self, path: Path) -> None:
        self.path = path

    def load(self) -> dict[str, DocumentState]:
        """Return the stored state, or ``{}`` when the file does not exist."""
        if not self.path.is_file():
            return {}
        with self.path.open("r", encoding="utf-8") as handle:
            data = json.load(handle)
        return {
            document_id: DocumentState.model_validate(record)
            for document_id, record in data.items()
        }

    def save(self, state: Mapping[str, DocumentState]) -> None:
        """Persist ``state`` atomically."""
        payload = {
            document_id: record.model_dump(mode="json") for document_id, record in state.items()
        }
        atomic_write_json(self.path, payload)
