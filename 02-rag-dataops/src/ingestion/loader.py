from __future__ import annotations

import sqlite3
from pathlib import Path
from typing import Any

from pydantic import BaseModel, Field

from src.config import Settings
from src.ingestion.parser import ParsedDocument, parse_database_row, parse_file

SUPPORTED_EXTENSIONS = {".pdf", ".docx", ".html", ".htm", ".txt"}
_TABLE = "documents"
_PRIMARY_KEY = "id"
_PROJECTION_COLUMNS = ["title", "body"]


class SourceProfile(BaseModel):
    """Connector profile: how a source type is discovered and read."""

    name: str
    connector: str  # filesystem | web | confluence | sharepoint | database
    config: dict[str, Any] = Field(default_factory=dict)


FILESYSTEM_PROFILE = SourceProfile(
    name="filesystem",
    connector="filesystem",
    config={"path": "data/raw", "extensions": sorted(SUPPORTED_EXTENSIONS)},
)
WEB_PROFILE = SourceProfile(
    name="web",
    connector="web",
    config={
        "urls": [],
        "notes": "URL list as a source profile; production connector only (LlamaIndex web reader).",
    },
)
CONFLUENCE_PROFILE = SourceProfile(
    name="confluence",
    connector="confluence",
    config={
        "notes": "HTML export / web reader; registered as a profile, not wired in the local MVP.",
    },
)
SHAREPOINT_PROFILE = SourceProfile(
    name="sharepoint",
    connector="sharepoint",
    config={
        "notes": "File download / web reader; registered as a profile, not wired in the local MVP.",
    },
)
DATABASE_PROFILE = SourceProfile(
    name="database",
    connector="database",
    config={
        "path": "raw/sample.db",  # relative to the data root (parent of data/raw)
        "table": _TABLE,
        "primary_key": _PRIMARY_KEY,
        "projection_columns": _PROJECTION_COLUMNS,
        "notes": "Read-only SQLite connector (local MVP)",
    },
)

SOURCE_PROFILES: dict[str, SourceProfile] = {
    profile.name: profile
    for profile in (
        FILESYSTEM_PROFILE,
        WEB_PROFILE,
        CONFLUENCE_PROFILE,
        SHAREPOINT_PROFILE,
        DATABASE_PROFILE,
    )
}


class DiscoveredDocument(BaseModel):
    """A source document found by a connector, not yet parsed."""

    document_id: str
    source: str
    format: str
    path: Path | None = None
    metadata: dict[str, Any] = Field(default_factory=dict)
    text: str | None = None  # pre-projected text (database connector)


class Loader:
    """Discover documents per source profile and route them to parsers."""

    def __init__(self, settings: Settings):
        self.settings = settings
        self.profiles = SOURCE_PROFILES

    def _filesystem_documents(self, data_dir: Path) -> list[DiscoveredDocument]:
        documents: list[DiscoveredDocument] = []
        if not data_dir.exists():
            return documents
        for path in sorted(data_dir.rglob("*")):
            if path.is_file() and path.suffix.lower() in SUPPORTED_EXTENSIONS:
                documents.append(
                    DiscoveredDocument(
                        document_id=path.relative_to(data_dir).as_posix(),
                        source="filesystem",
                        format=path.suffix.lstrip(".").lower(),
                        path=path,
                        metadata={"path": str(path)},
                    )
                )
        return documents

    def _database_documents(self) -> list[DiscoveredDocument]:
        profile = SOURCE_PROFILES["database"]
        db_path = Path(profile.config["path"])
        if not db_path.is_absolute():
            db_path = self.settings.data_dir.parent / db_path
        if not db_path.exists():
            return []
        table = profile.config["table"]
        primary_key = profile.config["primary_key"]
        query = f"SELECT * FROM {table}"
        with sqlite3.connect(db_path) as connection:
            connection.row_factory = sqlite3.Row
            rows = connection.execute(query).fetchall()
        documents: list[DiscoveredDocument] = []
        for row in rows:
            item = dict(row)
            documents.append(
                DiscoveredDocument(
                    document_id=f"{table}/{item[primary_key]}",
                    source="database",
                    format="database_row",
                    text=None,
                    metadata={
                        "table": table,
                        "primary_key": primary_key,
                        "row": item,
                    },
                )
            )
        return documents

    def discover(self, *profile_names: str) -> list[DiscoveredDocument]:
        """Discover documents for the given profiles (default: all wired ones)."""
        names = profile_names or ("filesystem", "database")
        found: list[DiscoveredDocument] = []
        for name in names:
            profile = SOURCE_PROFILES[name]
            if profile.connector == "filesystem":
                found.extend(self._filesystem_documents(self.settings.data_dir))
            elif profile.connector == "database":
                found.extend(self._database_documents())
        return sorted(found, key=lambda document: document.document_id)

    def load(self, document: DiscoveredDocument) -> ParsedDocument:
        """Route a discovered document to the matching parser."""
        if document.source == "database":
            return parse_database_row(
                table=document.metadata["table"],
                primary_key=document.metadata["primary_key"],
                row=document.metadata["row"],
                document_id=document.document_id,
            )
        if document.path is None:
            raise ValueError(f"Document {document.document_id!r} has no source path")
        return parse_file(document.path, document.document_id, source=document.source)