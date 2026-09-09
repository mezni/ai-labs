from __future__ import annotations

import pytest
from llama_index.core.schema import TextNode

from src.indexing.builder import Builder, BuildError
from src.indexing.registry import IndexRegistry
from tests.conftest import text_vector


def make_node(text, document_id="a.pdf", source="filesystem", format="pdf"):
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
            "chunk_hash": "h",
        },
    )
    node.embedding = text_vector(text)
    return node


def test_build_creates_snapshot_and_registers_version(settings, fake_embedder):
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)

    nodes = [
        make_node("alpha", "a.pdf", source="pdf"),
        make_node("beta", "a.pdf"),
        make_node("gamma", "b.docx", source="docx"),
    ]
    build = builder.build(version="v1", registry=registry, embedder=fake_embedder, nodes=nodes)

    assert build.version == "v1"
    assert build.collection_name == "docs_v1"
    assert build.chunk_count == 3
    assert build.document_count == 2
    assert build.stats == {"pdf": 1, "docx": 1}
    assert registry.current_version == "v1"
    config = registry.versions["v1"]
    assert config.embedding_model == "test-embed"
    assert config.embedding_dimension == 4
    assert config.chunk_size == settings.chunk_size

    store = builder._store("v1")
    loaded = store.get_nodes([node.id_ for node in nodes])
    assert len(loaded) == 3
    for node in loaded:
        assert node.metadata["index_version"] == "v1"


def test_build_without_embeddings_fails_and_does_not_mutate_registry(
    settings, fake_embedder
):
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)
    node = TextNode(text="x", id_="x::v1::chunk::0", metadata={"document_id": "x"})  # no embedding

    with pytest.raises(BuildError):
        builder.build(version="v1", registry=registry, embedder=fake_embedder, nodes=[node])
    assert registry.current_version is None
    assert not registry.versions
    assert not settings.index_registry.exists()


def test_carried_nodes_are_re_stamped(settings, fake_embedder):
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)
    first = builder.build(
        version="v1",
        registry=registry,
        embedder=fake_embedder,
        nodes=[make_node("alpha", "a.pdf")],
    )
    second = builder.build(
        version="v2",
        registry=registry,
        embedder=fake_embedder,
        nodes=[],
    )
    assert second.version == "v2"
    assert registry.current_version == "v2"
    assert registry.versions["v1"]["collection_name"] == first.collection_name
    assert IndexRegistry().load(settings.index_registry).current_version == "v2"


def test_next_build_does_not_overwrite_previous_snapshot(settings, fake_embedder):
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)
    builder.build(version="v1", registry=registry, embedder=fake_embedder, nodes=[make_node("a")])
    assert (settings.chroma_versions_dir / "v1").is_dir()
    builder.build(version="v2", registry=registry, embedder=fake_embedder, nodes=[make_node("b")])
    assert (settings.chroma_versions_dir / "v1").is_dir()
    assert (settings.chroma_versions_dir / "v2").is_dir()