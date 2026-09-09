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