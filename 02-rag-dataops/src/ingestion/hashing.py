from __future__ import annotations

import hashlib
import json
import logging
from enum import StrEnum
from pathlib import Path

from pydantic import BaseModel, Field

logger = logging.getLogger("rag.dataops.hashing")


class Change(StrEnum):
    NEW = "NEW"
    UNCHANGED = "UNCHANGED"
    CHANGED = "CHANGED"
    DELETED = "DELETED"


def sha256_stream(stream) -> str:
    """Streaming SHA-256 of a binary file-like object."""
    digest = hashlib.sha256()
    for chunk in iter(lambda: stream.read(1024 * 1024), b""):
        digest.update(chunk)
    return digest.hexdigest()


def sha256_file(path: Path) -> str:
    with path.open("rb") as handle:
        return sha256_stream(handle)


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def bump_version(record: DocumentRecord | None) -> str:
    """Next version for a document from its hash history, e.g. v4.

    A brand-new document starts at ``v1``; every change moves the version up by
    one, so lineage can be traced across index snapshots.
    """
    if record is None:
        return "v1"
    numbers = [int(record.version[1:])] if record.version.startswith("v") else []
    numbers.extend(int(v.version[1:]) for v in record.history if v.version.startswith("v"))
    return f"v{max(numbers, default=0) + 1}"


def detect_changes(
    current: dict[str, str], previous: dict[str, str]
) -> dict[str, Change]:
    """Classify every document id into NEW / UNCHANGED / CHANGED / DELETED.

    ``current`` maps document_id → current hash, ``previous`` maps document_id →
    the last recorded hash. A catalog that never existed means everything is NEW.
    """
    changes: dict[str, Change] = {}
    previous_ids = set(previous)
    for doc_id, hash_ in current.items():
        if doc_id not in previous:
            changes[doc_id] = Change.NEW
        elif previous[doc_id] == hash_:
            changes[doc_id] = Change.UNCHANGED
        else:
            changes[doc_id] = Change.CHANGED
    for doc_id in previous_ids - set(current):
        changes[doc_id] = Change.DELETED
    return changes


class DocumentVersion(BaseModel):
    """One known hash of a document, for lineage across snapshots."""

    version: str
    hash: str
    detected_at: str = ""


class DocumentRecord(BaseModel):
    """Per-document state stored in ``document_catalog.json``."""

    version: str = "v1"
    hash: str = ""
    last_modified: str | None = None
    source: str = ""
    format: str = ""
    chunk_ids: list[str] = Field(default_factory=list)
    history: list[DocumentVersion] = Field(default_factory=list)

    @property
    def versions(self) -> list[str]:
        return [v.version for v in self.history] + [self.version]


class DocumentCatalog(BaseModel):
    """The document catalog: document_id → DocumentRecord.

    Persisted to ``indexes/document_catalog.json`` and drives change detection,
    document versioning, and lineage lookup.
    """

    records: dict[str, DocumentRecord] = Field(default_factory=dict)

    def load(self, path: Path) -> DocumentCatalog:
        path = Path(path)
        if not path.exists():
            return DocumentCatalog()
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except (json.JSONDecodeError, OSError) as exc:
            logger.error("Failed to read document catalog %s: %s", path, exc)
            return DocumentCatalog()
        if isinstance(data, dict) and "records" in data and isinstance(data["records"], dict):
            data = data["records"]  # tolerate the older model_dump_json shape
        return DocumentCatalog(records=data if isinstance(data, dict) else {})

    def save(self, path: Path) -> None:
        path = Path(path)
        path.parent.mkdir(parents=True, exist_ok=True)
        tmp = path.with_suffix(".tmp")
        records = {doc_id: record.model_dump() for doc_id, record in self.records.items()}
        tmp.write_text(json.dumps(records, indent=2), encoding="utf-8")
        tmp.replace(path)

    def hashes(self) -> dict[str, str]:
        return {doc_id: record.hash for doc_id, record in self.records.items()}

    def get(self, document_id: str) -> DocumentRecord | None:
        return self.records.get(document_id)

    def set(self, document_id: str, record: DocumentRecord) -> None:
        self.records[document_id] = record

    def remove(self, document_id: str) -> None:
        self.records.pop(document_id, None)

    def items(self) -> list[tuple[str, DocumentRecord]]:
        return sorted(self.records.items(), key=lambda item: item[0])

    def record_detection(
        self, document_id: str, change: Change, hash_: str, detected_at: str
    ) -> DocumentRecord:
        """Apply a change-detection outcome to the catalog.

        NEW/CHANGED bump the version and append the old hash to history;
        UNCHANGED leaves the record untouched; DELETED removes the entry.
        """
        prior = self.get(document_id)
        if change is Change.DELETED:
            self.remove(document_id)
            record = prior if prior is not None else DocumentRecord(hash=hash_)
            record.chunk_ids = []
            return record
        if change is Change.UNCHANGED and prior is not None:
            return prior
        version = bump_version(prior)
        record = DocumentRecord(
            version=version,
            hash=hash_,
            last_modified=prior.last_modified if prior else None,
            source=prior.source if prior else "",
            format=prior.format if prior else "",
        )
        if prior is not None:
            record.history = list(prior.history) + [
                DocumentVersion(version=prior.version, hash=prior.hash, detected_at=detected_at)
            ]
        self.set(document_id, record)
        return record