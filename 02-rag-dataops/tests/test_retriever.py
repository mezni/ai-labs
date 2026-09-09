from __future__ import annotations

import pytest
from llama_index.core.schema import TextNode

from src.embeddings.embedder import Embedder
from src.indexing.builder import Builder
from src.indexing.registry import IndexRegistry, IndexVersionConfig
from src.retrieval.retriever import RetrievalError, Retriever
from tests.conftest import FakeEmbeddingsClient, text_vector


def build_snapshot(settings, fake_embedder, texts=None):
    texts = texts or [
        "the refund policy allows returns within 45 days",
        "remote work is allowed on fridays",
        "backups run nightly at 02:00 UTC",
    ]
    builder = Builder(settings)
    registry = IndexRegistry().load(settings.index_registry)
    nodes = []
    for index, text in enumerate(texts):
        node = TextNode(
            text=text,
            id_=f"doc-{index}::v1::chunk::0",
            metadata={
                "document_id": f"doc-{index}.pdf",
                "version": "v1",
                "source": "filesystem",
                "format": "pdf",
                "chunk_index": 0,
                "index_version": "v1",
            },
        )
        node.embedding = text_vector(text)
        nodes.append(node)
    builder.build(version="v1", registry=registry, embedder=fake_embedder, nodes=nodes)
    return registry


def test_retrieve_pulls_top_k_with_lineage(settings, fake_embedder):
    texts = [
        "the refund policy allows returns within 45 days",
        "remote work is allowed on fridays",
        "backups run nightly at 02:00 UTC",
    ]
    build_snapshot(settings, fake_embedder, texts)
    retriever = Retriever(settings, fake_embedder)
    results = retriever.retrieve(texts[0], top_k=2)
    assert results
    assert results[0].chunk_id == "doc-0::v1::chunk::0"
    assert results[0].lineage["document_id"] == "doc-0.pdf"
    assert results[0].lineage["index_version"] == "v1"
    assert results[0].text == texts[0]


def test_retrieve_model_mismatch_raises(settings, fake_embedder):
    build_snapshot(settings, fake_embedder)
    other = Embedder(
        api_key="k",
        base_url="https://x",
        model="different-model",
        client=FakeEmbeddingsClient(),
    )
    with pytest.raises(RetrievalError, match="does not match"):
        Retriever(settings, other).retrieve("query")


def test_retrieve_without_current_version_raises(settings, fake_embedder):
    with pytest.raises(RetrievalError, match="No current index version"):
        Retriever(settings, fake_embedder).retrieve("query")


def test_retrieve_missing_snapshot_dir_raises(settings, fake_embedder):
    registry = IndexRegistry()
    registry.register(
        "v3",
        IndexVersionConfig(
            collection_name="docs_v3",
            embedding_model="test-embed",
            chunk_count=1,
        ),
    )
    registry.set_current("v3")
    registry.save(settings.index_registry)
    with pytest.raises(RetrievalError, match="missing on disk"):
        Retriever(settings, fake_embedder).retrieve("query")


def test_top_k_honoured(settings, fake_embedder):
    texts = ["A", "B", "C"]
    build_snapshot(settings, fake_embedder, texts)
    retriever = Retriever(settings, fake_embedder)
    assert len(retriever.retrieve("A", top_k=1)) == 1
    assert len(retriever.retrieve("A", top_k=5)) == 3