from __future__ import annotations

import logging
from pathlib import Path

import chromadb

from src.config import Settings
from src.indexing.registry import IndexRegistry

logger = logging.getLogger("rag.dataops.versioning")


class RollbackError(Exception):
    """Rollback failed: the target snapshot is missing or invalid."""


class Versioning:
    """Snapshot lifecycle + rollback (PROJECT.md §12)."""

    def __init__(self, settings: Settings):
        self.settings = settings
        self.registry = IndexRegistry().load(settings.index_registry)

    def snapshot_path(self, version: str) -> Path:
        return self.settings.chroma_versions_dir / version

    def _load_registry(self) -> IndexRegistry:
        return IndexRegistry().load(self.settings.index_registry)

    def snapshot_exists(self, version: str) -> bool:
        """A snapshot exists if its Chroma store path exists on disk."""
        return self.snapshot_path(version).is_dir()

    def validate_snapshot(self, version: str, collection_name: str) -> int:
        """Open the snapshot store and return its current chunk count.

        Raises RollbackError if the snapshot is missing from disk, its Chroma
        store does not open, or the stored collection is not present.
        """
        path = self.snapshot_path(version)
        if not self.snapshot_exists(version):
            raise RollbackError(f"Rollback failed: version {version} snapshot missing")
        try:
            client = chromadb.PersistentClient(path=str(path))
            collection = client.get_collection(collection_name)
        except Exception as exc:
            raise RollbackError(
                f"Rollback failed: version {version} collection {collection_name!r} "
                f"missing or unreadable"
            ) from exc
        return collection.count()

    def rollback(self, version: str) -> IndexRegistry:
        """Point ``current_version`` back to a stored snapshot (physical restore).

        The target snapshot must exist on disk and its collection must be
        readable; otherwise the registry is left untouched.
        """
        registry = self._load_registry()
        if version not in registry.versions:
            raise RollbackError(f"Rollback failed: version {version} not in registry")
        if version == registry.current_version:
            logger.info("Already on index version %s; nothing to roll back", version)
            return registry
        config = registry.versions[version]
        count = self.validate_snapshot(version, config.collection_name)
        registry.set_current(version)
        registry.save(self.settings.index_registry)
        logger.info(
            "Rolled back to index version %s (collection=%s, chunks=%s)",
            version,
            config.collection_name,
            count,
        )
        return registry