from __future__ import annotations

import importlib
import json

import pandas as pd

import app.artifacts as artifacts
from app.app import registry_frame
from app.pages import _1_Documents

REGISTRY = {
    "current_version": "v2",
    "versions": {
        "v1": {
            "collection_name": "docs_v1",
            "chunk_count": 10,
            "document_count": 2,
            "embedding_model": "m",
            "embedding_dimension": 4,
            "stats": {"pdf": 8, "txt": 2},
        },
        "v2": {
            "collection_name": "docs_v2",
            "chunk_count": 12,
            "document_count": 2,
            "embedding_model": "m",
            "embedding_dimension": 4,
            "stats": {"pdf": 9, "txt": 3},
        },
    },
}


def test_source_counts_uses_current_version():
    assert artifacts.source_counts(REGISTRY) == {"pdf": 9, "txt": 3}
    assert artifacts.source_counts({"current_version": None, "versions": {}}) == {}
    assert artifacts.source_counts({"current_version": "v9", "versions": {}}) == {}


def test_load_json_missing_and_corrupt(tmp_path):
    assert artifacts.load_json(tmp_path / "missing.json") == {}
    bad = tmp_path / "bad.json"
    bad.write_text("{nope")
    assert artifacts.load_json(bad) == {}


def test_load_registry_and_catalog_from_tmp_root(tmp_path, monkeypatch):
    indexes = tmp_path / "indexes"
    indexes.mkdir()
    (indexes / "index_registry.json").write_text(json.dumps(REGISTRY), encoding="utf-8")
    (indexes / "document_catalog.json").write_text(
        json.dumps({"a.pdf": {"version": "v2", "hash": "h", "source": "filesystem"}}),
        encoding="utf-8",
    )
    monkeypatch.setattr(artifacts, "PROJECT_ROOT", tmp_path)
    assert artifacts.load_registry()["current_version"] == "v2"
    assert artifacts.load_catalog()["a.pdf"]["source"] == "filesystem"


def test_registry_frame_rows():
    frame = registry_frame(REGISTRY)
    assert isinstance(frame, pd.DataFrame)
    assert len(frame) == 2
    assert list(frame["version"]) == ["v1", "v2"]
    assert frame.iloc[1]["chunks"] == 12


def test_json_round_trip(tmp_path):
    path = tmp_path / "indexes"
    path.mkdir()
    target = path / "index_registry.json"
    target.write_text(json.dumps(REGISTRY), encoding="utf-8")
    assert artifacts.load_json(target) == REGISTRY


def test_catalog_frame_projections():
    catalog = {
        "a.pdf": {
            "version": "v2",
            "hash": "h1",
            "source": "filesystem",
            "format": "pdf",
            "last_modified": None,
            "chunk_ids": ["x", "y"],
        }
    }
    frame = _1_Documents.catalog_frame(catalog)
    assert len(frame) == 1
    assert frame.iloc[0]["chunks"] == 2
    assert _1_Documents.catalog_frame({}).empty


def test_page_modules_importable_and_expose_render():
    for name in (
        "1_Documents",
        "2_Index_Versions",
        "3_Lineage",
        "4_Search",
        "5_Rollback",
    ):
        module = importlib.import_module(f"app.pages.{name}")
        assert callable(getattr(module, "render", None))