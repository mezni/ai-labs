from __future__ import annotations

import json

from src.ingestion.hashing import (
    Change,
    DocumentCatalog,
    DocumentRecord,
    DocumentVersion,
    bump_version,
    detect_changes,
    sha256_file,
    sha256_stream,
    sha256_text,
)


def hash_file(tmp_path, content: bytes, name="doc.txt"):
    path = tmp_path / name
    path.write_bytes(content)
    return path


def test_sha256_text_deterministic():
    first = sha256_text("hello world")
    second = sha256_text("hello world")
    assert first == second
    assert sha256_text("hello world!") != first


def test_sha256_stream_and_file_consistent(tmp_path):
    content = b"the quick brown fox" * 1024 * 4
    path = hash_file(tmp_path, content)
    with path.open("rb") as handle:
        assert sha256_stream(handle) == sha256_file(path)


def test_change_detection_all_four_cases():
    current = {
        "new.pdf": "h1",
        "same.pdf": "sig-a",
        "changed.pdf": "sig-b",
    }
    previous = {
        "same.pdf": "sig-a",
        "changed.pdf": "sig-old",
        "deleted.pdf": "sig-c",
    }
    changes = detect_changes(current, previous)
    assert changes["new.pdf"] is Change.NEW
    assert changes["same.pdf"] is Change.UNCHANGED
    assert changes["changed.pdf"] is Change.CHANGED
    assert changes["deleted.pdf"] is Change.DELETED


def test_detect_changes_empty_previous_means_all_new():
    changes = detect_changes({"a": "h"}, {})
    assert changes == {"a": Change.NEW}


def test_bump_version():
    assert bump_version(None) == "v1"
    assert bump_version(DocumentRecord(version="v4", hash="x")) == "v5"
    record = DocumentRecord(
        version="v2",
        hash="x",
        history=[DocumentVersion(version="v1", hash="a")],
    )
    assert bump_version(record) == "v3"


def test_catalog_round_trip(tmp_path):
    path = tmp_path / "document_catalog.json"
    catalog = DocumentCatalog()
    catalog.set("a.pdf", DocumentRecord(version="v1", hash="h1", source="filesystem"))
    catalog.save(path)
    assert path.exists()
    loaded = DocumentCatalog().load(path)
    assert loaded.get("a.pdf").version == "v1"
    assert loaded.get("a.pdf").hash == "h1"


def test_catalog_load_missing_file_returns_empty(tmp_path):
    assert DocumentCatalog().load(tmp_path / "missing.json").records == {}


def test_catalog_load_corrupt_file_returns_empty(tmp_path):
    path = tmp_path / "bad.json"
    path.write_text("{not json")
    assert DocumentCatalog().load(path).records == {}


def test_record_detection_transitions():
    catalog = DocumentCatalog()
    new = catalog.record_detection("a.pdf", Change.NEW, "h1", detected_at="v1")
    assert new.version == "v1"
    assert new.hash == "h1"

    same = catalog.record_detection("a.pdf", Change.UNCHANGED, "h1", detected_at="v2")
    assert same is new
    assert catalog.get("a.pdf").version == "v1"

    changed = catalog.record_detection("a.pdf", Change.CHANGED, "h2", detected_at="v2")
    assert changed.version == "v2"
    assert changed.hash == "h2"
    assert changed.history[-1].version == "v1"
    assert changed.history[-1].hash == "h1"

    catalog.record_detection("a.pdf", Change.DELETED, "", detected_at="v3")
    assert catalog.get("a.pdf") is None


def test_idempotency_no_state_change_on_unchanged():
    catalog = DocumentCatalog()
    catalog.record_detection("a", Change.NEW, "h", detected_at="v1")
    before = catalog.model_dump_json()
    catalog.record_detection("a", Change.UNCHANGED, "h", detected_at="v2")
    assert catalog.model_dump_json() == before


def test_history_persisted_in_catalog_file(tmp_path):
    path = tmp_path / "catalog.json"
    catalog = DocumentCatalog()
    catalog.record_detection("a", Change.NEW, "h1", detected_at="v1")
    catalog.record_detection("a", Change.CHANGED, "h2", detected_at="v2")
    catalog.save(path)
    raw = json.loads(path.read_text())
    assert raw["a"]["version"] == "v2"
    # the v1 hash is superseded at the version that replaced it (v2)
    assert raw["a"]["history"] == [{"version": "v1", "hash": "h1", "detected_at": "v2"}]


def test_catalog_hashes_view():
    catalog = DocumentCatalog()
    catalog.set("a", DocumentRecord(version="v1", hash="ha"))
    catalog.set("b", DocumentRecord(version="v1", hash="hb"))
    assert catalog.hashes() == {"a": "ha", "b": "hb"}