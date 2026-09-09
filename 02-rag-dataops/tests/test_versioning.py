from __future__ import annotations

import pytest

from src.indexing.builder import Builder
from src.indexing.registry import IndexRegistry, IndexVersionConfig
from src.indexing.versioning import RollbackError, Versioning


def make_node(text, document_id="a.pdf", source="filesystem", format="pdf"):
    from llama_index.core.schema import TextNode
    import hashlib

    node = TextNode(
        text=text,
        id_=f"{document_id}::v1::chunk::0",
        metadata={
            "document_id": document_id,
            "version": "v1",
            "source": source,
            "format": format,
            "chunk_index": 0,
            "block_start": 0,
            "block_end": len(text),
            "chunk_hash": hashlib.sha256(text.encode("utf-8")).hexdigest(),
        },
    )
    node.embedding = [int(hashlib.sha256(text.encode("utf-8")).hexdigest()[i : i + 2], 16) / 255 for i in range(0, 4 * 2, 2)]
    return node


@pytest.fixture
def two_snapshots(settings, fake_embedder):
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)
    builder.build(version="v1", registry=registry, embedder=fake_embedder, nodes=[make_node("old doc", "a.pdf")])
    builder.build(version="v2", registry=registry, embedder=fake_embedder, nodes=[make_node("new doc", "a.pdf")])
    return settings, registry


def test_current_version_is_latest(two_snapshots):
    settings, _ = two_snapshots
    registry = IndexRegistry().load(settings.index_registry)
    assert registry.current_version == "v2"


def test_rollback_repoints_current(two_snapshots):
    settings, _ = two_snapshots
    versioning = Versioning(settings)
    versioning.rollback("v1")
    registry = IndexRegistry().load(settings.index_registry)
    assert registry.current_version == "v1"
    assert set(registry.versions) == {"v1", "v2"}


def test_rollback_to_unknown_version_fails_loudly(settings):
    with pytest.raises(RollbackError):
        Versioning(settings).rollback("v99")


def test_rollback_missing_snapshot_fails_and_leaves_state(settings):
    registry = IndexRegistry()
    registry.register(
        "v1",
        IndexVersionConfig(collection_name="docs_v1", embedding_model="m", chunk_count=1),
    )
    registry.current_version = None
    registry.save(settings.index_registry)

    versioning = Versioning(settings)
    with pytest.raises(RollbackError):
        versioning.rollback("v1")
    reloaded = IndexRegistry().load(settings.index_registry)
    assert reloaded.current_version is None


def test_rollback_missing_snapshot_dir_fails_loudly(settings, fake_embedder):
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)
    builder.build(version="v1", registry=registry, embedder=fake_embedder, nodes=[make_node("x", "a.pdf")])
    # v2 registered but its snapshot directory was never built
    config_v2 = registry.versions["v1"].model_copy(update={"collection_name": "docs_v2"})
    registry.register("v2", config_v2)
    registry.save(settings.index_registry)

    versioning = Versioning(settings)
    with pytest.raises(RollbackError):
        versioning.rollback("v2")
    reloaded = IndexRegistry().load(settings.index_registry)
    assert reloaded.current_version == "v1"


def test_rollback_validate_snapshot_count(settings, fake_embedder):
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)
    build = builder.build(version="v1", registry=registry, embedder=fake_embedder, nodes=[make_node("x", "a.pdf")])
    versioning = Versioning(settings)
    assert versioning.snapshot_exists("v1")
    assert versioning.validate_snapshot("v1", build.collection_name) == 1