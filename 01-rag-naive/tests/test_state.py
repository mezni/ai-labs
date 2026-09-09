"""Tests for ``rag.state`` hashing, change detection, and state persistence."""

import json

import pytest
from pydantic import ValidationError

from rag.state import (
    Change,
    DocumentState,
    DocumentStateStore,
    atomic_write_json,
    detect_changes,
    hash_documents,
    sha256_file,
)
from tests.pdf_factory import build_pdf


def _write_pdf(directory, name, pages):
    path = directory / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(build_pdf(pages))
    return path


def test_sha256_file_is_deterministic_and_hex(tmp_path):
    path = tmp_path / "policy.pdf"
    path.write_bytes(b"should become a hash")

    first = sha256_file(path)
    second = sha256_file(path)

    assert first == second
    assert len(first) == 64


def test_sha256_file_changes_with_content(tmp_path):
    a = tmp_path / "a.pdf"
    b = tmp_path / "b.pdf"
    a.write_bytes(b"version one")
    b.write_bytes(b"version two")

    assert sha256_file(a) != sha256_file(b)


def test_sha256_file_hashes_large_content(tmp_path):
    path = tmp_path / "big.pdf"
    path.write_bytes(b"x" * 200_000)

    assert len(sha256_file(path)) == 64


def test_hash_documents_maps_relative_pdf_paths(tmp_path):
    _write_pdf(tmp_path, "billing_policy.pdf", ["billing"])
    _write_pdf(tmp_path / "sub", "roaming_policy.PDF", ["roaming"])
    _write_pdf(tmp_path, "notes.txt", ["not indexed"])

    hashes = hash_documents(tmp_path)

    assert set(hashes) == {"billing_policy.pdf", "sub/roaming_policy.PDF"}
    assert all(len(digest) == 64 for digest in hashes.values())
    assert hashes["billing_policy.pdf"] == sha256_file(tmp_path / "billing_policy.pdf")


def test_detect_changes_all_new():
    current = {"billing_policy.pdf": "aa"}

    changes = detect_changes(current, {})

    assert changes == {"billing_policy.pdf": Change.NEW}


def test_detect_changes_unchanged():
    current = {"billing_policy.pdf": "aa"}
    previous = {"billing_policy.pdf": DocumentState(hash="aa", vector_ids=[1], chunk_count=1)}

    changes = detect_changes(current, previous)

    assert changes == {"billing_policy.pdf": Change.UNCHANGED}


def test_detect_changes_changed():
    current = {"billing_policy.pdf": "bb"}
    previous = {"billing_policy.pdf": DocumentState(hash="aa", vector_ids=[1], chunk_count=1)}

    changes = detect_changes(current, previous)

    assert changes == {"billing_policy.pdf": Change.CHANGED}


def test_detect_changes_deleted():
    previous = {"old_policy.pdf": DocumentState(hash="aa", vector_ids=[1], chunk_count=1)}

    changes = detect_changes({}, previous)

    assert changes == {"old_policy.pdf": Change.DELETED}


def test_detect_changes_mixed():
    previous = {
        "unchanged.pdf": DocumentState(hash="aa", vector_ids=[1], chunk_count=1),
        "changed.pdf": DocumentState(hash="aa", vector_ids=[2], chunk_count=1),
        "deleted.pdf": DocumentState(hash="aa", vector_ids=[3], chunk_count=1),
    }
    current = {
        "unchanged.pdf": "aa",
        "changed.pdf": "bb",
        "new.pdf": "cc",
    }

    changes = detect_changes(current, previous)

    assert changes == {
        "unchanged.pdf": Change.UNCHANGED,
        "changed.pdf": Change.CHANGED,
        "new.pdf": Change.NEW,
        "deleted.pdf": Change.DELETED,
    }


def test_document_state_store_roundtrip(tmp_path):
    path = tmp_path / "document_state.json"
    store = DocumentStateStore(path)
    state = {
        "billing_policy.pdf": DocumentState(hash="aa", vector_ids=[1000, 1001], chunk_count=2),
        "roaming_policy.pdf": DocumentState(hash="bb", vector_ids=[1002], chunk_count=1),
    }

    store.save(state)
    loaded = DocumentStateStore(path).load()

    assert loaded == state


def test_document_state_store_load_missing_returns_empty(tmp_path):
    store = DocumentStateStore(tmp_path / "does-not-exist.json")

    assert store.load() == {}


def test_atomic_write_json_writes_and_replaces(tmp_path):
    path = tmp_path / "state.json"

    atomic_write_json(path, {"a": 1})
    atomic_write_json(path, {"b": 2})

    with path.open("r", encoding="utf-8") as handle:
        assert json.load(handle) == {"b": 2}
    assert not list(tmp_path.glob("*.tmp"))


def test_document_state_rejects_negative_chunk_count():
    with pytest.raises(ValidationError):
        DocumentState(hash="aa", vector_ids=[], chunk_count=-1)


def test_document_state_model_dump_shape():
    record = DocumentState(hash="aa", vector_ids=[1, 2], chunk_count=2)

    data = record.model_dump(mode="json")

    assert set(data) == {"hash", "vector_ids", "chunk_count"}
    assert data["hash"] == "aa"
    assert data["vector_ids"] == [1, 2]
    assert data["chunk_count"] == 2
