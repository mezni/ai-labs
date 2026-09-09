from __future__ import annotations

import json

import pytest

from src.indexing.registry import IndexRegistry, IndexVersionConfig, VersionConflictError


def make_config(version="v1", chunk_count=10):
    return IndexVersionConfig(
        created_at="2026-09-01T00:00:00Z",
        embedding_model="test-embed",
        embedding_dimension=4,
        embedding_provider="https://gateway.test",
        chunk_size=800,
        chunk_overlap=150,
        collection_name=f"docs_{version}",
        chunk_count=chunk_count,
        document_count=2,
        stats={"pdf": 8},
    )


def test_empty_registry(tmp_path):
    registry = IndexRegistry().load(tmp_path / "missing.json")
    assert registry.current_version is None
    assert registry.versions == {}


def test_register_and_current(settings):
    registry = IndexRegistry().load(settings.index_registry)
    assert registry.next_version() == "v1"
    registry.register("v1", make_config("v1"))
    registry.set_current("v1")
    assert registry.current().collection_name == "docs_v1"
    assert registry.next_version() == "v2"


def test_registry_round_trip(settings):
    registry = IndexRegistry()
    registry.register("v1", make_config("v1"))
    registry.register("v2", make_config("v2"))
    registry.set_current("v2")
    registry.save(settings.index_registry)

    loaded = IndexRegistry().load(settings.index_registry)
    assert loaded.current_version == "v2"
    assert set(loaded.versions) == {"v1", "v2"}
    assert loaded.versions["v1"].chunk_count == 10


def test_existing_version_records_are_immutable(settings):
    registry = IndexRegistry()
    registry.register("v1", make_config("v1"))
    with pytest.raises(VersionConflictError):
        registry.register("v1", make_config("v1", chunk_count=99))
    assert registry.versions["v1"].chunk_count == 10


def test_set_current_to_unknown_version_raises():
    registry = IndexRegistry()
    with pytest.raises(KeyError):
        registry.set_current("nope")


def test_registry_json_shape_matches_spec(settings):
    registry = IndexRegistry()
    registry.register("v7", make_config("v7", chunk_count=4832))
    registry.set_current("v7")
    registry.save(settings.index_registry)

    raw = json.loads(settings.index_registry.read_text())
    assert raw["current_version"] == "v7"
    config = raw["versions"]["v7"]
    assert config["collection_name"] == "docs_v7"
    assert config["chunk_count"] == 4832
    assert config["embedding_model"] == "test-embed"