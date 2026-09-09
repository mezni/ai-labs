from __future__ import annotations

import json
from pathlib import Path

from pydantic import BaseModel, Field


class VersionConflictError(Exception):
    """A version record already exists and is immutable."""


class IndexVersionConfig(BaseModel):
    """Per-index-version record (PROJECT.md §11). Never mutated after creation."""

    created_at: str = ""
    embedding_model: str = ""
    embedding_dimension: int = 0
    embedding_provider: str = ""
    chunk_size: int = 0
    chunk_overlap: int = 0
    collection_name: str = ""
    chunk_count: int = 0
    document_count: int = 0
    stats: dict[str, int] = Field(default_factory=dict)


class IndexRegistry(BaseModel):
    """Version catalog: ``current_version`` pointer + append-only version records."""

    current_version: str | None = None
    versions: dict[str, IndexVersionConfig] = Field(default_factory=dict)

    def load(self, path: Path) -> IndexRegistry:
        path = Path(path)
        if not path.exists():
            return IndexRegistry()
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except (json.JSONDecodeError, OSError):
            return IndexRegistry()
        return IndexRegistry.model_validate(data)

    def save(self, path: Path) -> None:
        path = Path(path)
        path.parent.mkdir(parents=True, exist_ok=True)
        tmp = path.with_suffix(".tmp")
        tmp.write_text(self.model_dump_json(indent=2), encoding="utf-8")
        tmp.replace(path)

    def register(self, version: str, config: IndexVersionConfig) -> None:
        """Append a new version record; existing records are immutable."""
        if version in self.versions:
            raise VersionConflictError(f"version {version} is already registered")
        self.versions[version] = config

    def current(self) -> IndexVersionConfig | None:
        if self.current_version is None:
            return None
        return self.versions.get(self.current_version)

    def set_current(self, version: str) -> None:
        if version not in self.versions:
            raise KeyError(f"version {version} is not registered")
        self.current_version = version

    def next_version(self) -> str:
        numbers = [int(v[1:]) for v in self.versions if v.startswith("v")]
        return f"v{max(numbers, default=0) + 1}"