"""Shared artifact readers for the Streamlit dashboard.

The dashboard reads exactly two artifacts (PROJECT.md §19 Data Hooks) and the
same services as the CLI — it never performs indexing itself.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from src.config import PROJECT_ROOT


def load_json(path: Path) -> Any:
    if not Path(path).exists():
        return {}
    try:
        return json.loads(Path(path).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def load_registry() -> dict[str, Any]:
    return load_json(PROJECT_ROOT / "indexes" / "index_registry.json")


def load_catalog() -> dict[str, Any]:
    return load_json(PROJECT_ROOT / "indexes" / "document_catalog.json")


def source_counts(registry: dict[str, Any]) -> dict[str, int]:
    """Per-source stats of the current version (empty dict when none yet)."""
    versions = registry.get("versions", {})
    current = registry.get("current_version")
    if current is None or current not in versions:
        return {}
    return versions[current].get("stats", {})