from __future__ import annotations

import logging
from datetime import UTC, datetime

import chromadb
from llama_index.core.schema import TextNode
from llama_index.vector_stores.chroma import ChromaVectorStore
from pydantic import BaseModel, Field

from src.config import Settings
from src.embeddings.embedder import Embedder
from src.indexing.registry import IndexRegistry, IndexVersionConfig

logger = logging.getLogger("rag.dataops.indexing")

_DIMENSION_MISSING = 0


class BuildError(Exception):
    """Snapshot build failed; the registry must not be mutated."""


class VersionBuild(BaseModel):
    """Result of a snapshot build (PROJECT.md §11)."""

    version: str
    collection_name: str
    chunk_count: int
    document_count: int
    stats: dict[str, int] = Field(default_factory=dict)
    embedding_dimension: int = _DIMENSION_MISSING


class Builder:
    """Build a new index snapshot into a versioned Chroma collection."""

    def __init__(self, settings: Settings):
        self.settings = settings

    def _store(self, version: str) -> ChromaVectorStore:
        path = self.settings.chroma_versions_dir / version
        path.mkdir(parents=True, exist_ok=True)
        client = chromadb.PersistentClient(path=str(path))
        collection = client.get_or_create_collection(
            name=f"docs_{version}", metadata={"hnsw:space": "cosine"}
        )
        return ChromaVectorStore(chroma_collection=collection)

    def read_nodes(
        self, version: str, collection_name: str, node_ids: list[str]
    ) -> dict[str, TextNode]:
        """Load chunks (with embeddings) from a stored snapshot collection."""
        path = self.settings.chroma_versions_dir / version
        client = chromadb.PersistentClient(path=str(path))
        collection = client.get_collection(collection_name)
        result = collection.get(
            ids=node_ids, include=["embeddings", "metadatas", "documents"]
        )
        nodes: dict[str, TextNode] = {}
        for node_id, text, metadata, embedding in zip(
            result["ids"],
            result["documents"],
            result["metadatas"],
            result["embeddings"],
            strict=False,
        ):
            metadata = {k: v for k, v in (metadata or {}).items() if v is not None}
            nodes[node_id] = TextNode(
                text=text or "",
                id_=node_id,
                embedding=embedding,
                metadata=metadata,
            )
        return nodes

    def build(
        self,
        *,
        version: str,
        registry: IndexRegistry,
        embedder: Embedder,
        nodes: list[TextNode],
    ) -> VersionBuild:
        """Persist ``nodes`` into ``version``'s collection.

        Every node is stamped with its ``index_version`` provenance field, then
        added. Failed builds raise BuildError and leave the registry untouched:
        the version record is only registered and ``current_version`` only
        repointed after the collection has accepted every chunk (PROJECT.md §18).
        """
        collection_name = f"docs_{version}"
        store = self._store(version)
        try:
            for node in nodes:
                node.metadata = {**node.metadata, "index_version": version}
                if node.embedding is None:
                    raise BuildError(
                        f"chunk {node.id_} has no embedding; refusing to build {version}"
                    )
            if nodes:
                store.add(nodes)
        except BuildError as exc:
            logger.error("Snapshot build failed; registry not mutated: %s", exc)
            raise exc
        except Exception as exc:
            logger.error(
                "Snapshot build failed; registry not mutated: %s", exc, exc_info=True
            )
            raise BuildError(f"Snapshot build failed: {exc}") from exc

        config = self._version_config(
            version=version,
            registry=registry,
            embedder=embedder,
            collection_name=collection_name,
            nodes=nodes,
        )
        registry.register(version, config)
        registry.set_current(version)
        registry.save(self.settings.index_registry)
        logger.info(
            "Built index snapshot %s (%s chunks, %s documents)",
            version,
            config.chunk_count,
            config.document_count,
        )
        return VersionBuild(
            version=version,
            collection_name=collection_name,
            chunk_count=config.chunk_count,
            document_count=config.document_count,
            stats=config.stats,
            embedding_dimension=config.embedding_dimension,
        )

    def _version_config(
        self,
        *,
        version: str,
        registry: IndexRegistry,
        embedder: Embedder,
        collection_name: str,
        nodes: list[TextNode],
    ) -> IndexVersionConfig:
        document_ids = sorted({node.metadata.get("document_id", "") for node in nodes})
        stats: dict[str, int] = {}
        for node in nodes:
            fmt = node.metadata.get("format", "unknown")
            stats[fmt] = stats.get(fmt, 0) + 1
        dimension = embedder.dimension
        if dimension is None:
            # infer from embedded content (e.g. carried/re-embedded nodes) when
            # the embedder has not captured the dimension yet
            for node in nodes:
                if node.embedding is not None:
                    dimension = len(node.embedding)
                    break
            if dimension is None:
                previous = registry.current()
                dimension = previous.embedding_dimension if previous else _DIMENSION_MISSING
        return IndexVersionConfig(
            created_at=datetime.now(UTC).isoformat(),
            embedding_model=embedder.model,
            embedding_dimension=dimension or _DIMENSION_MISSING,
            embedding_provider=embedder.base_url,
            chunk_size=self.settings.chunk_size,
            chunk_overlap=self.settings.chunk_overlap,
            collection_name=collection_name,
            chunk_count=len(nodes),
            document_count=len(document_ids),
            stats=stats,
        )