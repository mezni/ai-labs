from __future__ import annotations

import argparse
import logging
import sys

from src.chunking.chunker import Chunker
from src.config import Settings, get_settings
from src.embeddings.embedder import (
    DimensionMismatchError,
    Embedder,
    EmbeddingError,
    embedding_model_changed,
)
from src.indexing.builder import Builder
from src.indexing.registry import IndexRegistry
from src.indexing.versioning import Versioning
from src.ingestion.hashing import (
    Change,
    DocumentCatalog,
    bump_version,
    detect_changes,
    sha256_file,
    sha256_text,
)
from src.ingestion.loader import Loader
from src.ingestion.parser import ParseError, parse_database_row
from src.retrieval.retriever import RetrievalError, Retriever

logger = logging.getLogger("rag.dataops")


def configure_logging(settings: Settings, verbose: bool = False) -> None:
    """Structured logging to console and ``logs/rag-dataops.log`` (PROJECT.md §17)."""
    settings.log_path.parent.mkdir(parents=True, exist_ok=True)
    level = logging.DEBUG if verbose else logging.INFO
    formatter = logging.Formatter("%(asctime)s %(levelname)s %(name)s %(message)s")
    console = logging.StreamHandler()
    console.setFormatter(formatter)
    file_handler = logging.FileHandler(settings.log_path, encoding="utf-8")
    file_handler.setFormatter(formatter)
    root = logging.getLogger()
    root.setLevel(level)
    for handler in list(root.handlers):
        root.removeHandler(handler)
    root.addHandler(console)
    root.addHandler(file_handler)


def build_embedder(settings: Settings) -> Embedder:
    return Embedder(
        api_key=settings.openrouter_api_key,
        base_url=settings.openrouter_base_url,
        model=settings.openrouter_embedding_model,
    )


def _hash_for(discovered):
    """Content hash per discovered document (PROJECT.md §6)."""
    if discovered.source == "database":
        row = discovered.metadata["row"]
        table = discovered.metadata["table"]
        primary_key = discovered.metadata["primary_key"]
        projected = parse_database_row(
            table=table,
            primary_key=primary_key,
            row=row,
            document_id=discovered.document_id,
        )
        return sha256_text(projected.text)
    return sha256_file(discovered.path)


def run_index(settings: Settings) -> int:
    """`rag-dataops index`: discover → hash → change-detect → parse → chunk →
    embed → build snapshot → bump version (PROJECT.md §6/§19)."""
    settings.ensure_dirs()
    loader = Loader(settings)
    chunker = Chunker(settings.chunk_size, settings.chunk_overlap)
    embedder = build_embedder(settings)
    builder = Builder(settings)

    discovered = loader.discover()
    logger.info("Discovered %s documents", len(discovered))
    by_id = {doc.document_id: doc for doc in discovered}

    catalog = DocumentCatalog().load(settings.document_catalog)
    registry = IndexRegistry().load(settings.index_registry)
    current = registry.current()

    full_reindex = embedding_model_changed(
        current.embedding_model if current else None,
        settings.openrouter_embedding_model,
    )
    if full_reindex:
        logger.warning(
            "Embedding model changed %r -> %r; full reindex",
            current.embedding_model,
            settings.openrouter_embedding_model,
        )

    current_hashes = {doc.document_id: _hash_for(doc) for doc in discovered}
    changes = detect_changes(current_hashes, catalog.hashes())
    if full_reindex:
        changes = {doc_id: Change.NEW for doc_id in current_hashes}

    previous_version = registry.current_version
    previous_collection = current.collection_name if current else None

    deleted = [doc_id for doc_id in catalog.hashes() if doc_id not in current_hashes]

    real_change = any(
        change is Change.NEW or change is Change.CHANGED
        for change in changes.values()
    ) or bool(deleted)
    if not real_change:
        logger.info(
            "No changes (all %s documents unchanged); index is current — no new "
            "embeddings or snapshots",
            len(discovered),
        )
        return 0

    # document_id -> list[TextNode] ready to be persisted in the new snapshot
    snapshot_nodes: dict[str, list] = {}

    def embed_document(document_id: str) -> list | None:
        """Parse → chunk → embed one document; per-document failures are logged."""
        source_doc = by_id.get(document_id)
        if source_doc is None:
            logger.error("Document %s not discoverable; skipping", document_id)
            return None
        try:
            parsed = loader.load(source_doc)
        except ParseError as exc:
            logger.error("Parsing failed for %s: %s", document_id, exc)
            return None
        record = catalog.get(document_id)
        version = bump_version(record) if record else "v1"
        nodes = chunker.chunk(parsed, version=version)
        if not nodes:
            return []
        try:
            vectors = embedder.embed([node.text for node in nodes])
        except (EmbeddingError, DimensionMismatchError) as exc:
            logger.error("Embedding failed for %s: %s", document_id, exc)
            return None
        for node, vector in zip(nodes, vectors, strict=False):
            node.embedding = vector
        return nodes

    def carry_document(document_id: str) -> list | None:
        """Carry an UNCHANGED document's chunks from the current snapshot.

        No embedding request is made (PROJECT.md §6 idempotency). Falls back to
        re-embedding when the previous snapshot has no record of the chunks.
        """
        record = catalog.get(document_id)
        if (
            record is None
            or not record.chunk_ids
            or previous_version is None
            or previous_collection is None
        ):
            return embed_document(document_id)
        try:
            nodes = builder.read_nodes(
                previous_version, previous_collection, record.chunk_ids
            )
        except Exception as exc:
            logger.error(
                "Carry-over failed for %s from %s: %s",
                document_id,
                previous_version,
                exc,
            )
            return embed_document(document_id)
        missing = [node_id for node_id in record.chunk_ids if node_id not in nodes]
        if missing:
            logger.warning(
                "Re-embedding %s: %s chunks missing from snapshot %s",
                document_id,
                len(missing),
                previous_version,
            )
            return embed_document(document_id)
        return [nodes[node_id] for node_id in record.chunk_ids]

    logger.info("Parsed %s documents", len(discovered))
    for doc in discovered:
        change = changes.get(doc.document_id, Change.UNCHANGED)
        if change is Change.NEW:
            logger.info("NEW %s", doc.document_id)
            snapshot_nodes[doc.document_id] = embed_document(doc.document_id)
        elif change is Change.CHANGED:
            logger.info("CHANGED %s", doc.document_id)
            snapshot_nodes[doc.document_id] = embed_document(doc.document_id)
        else:
            snapshot_nodes[doc.document_id] = carry_document(doc.document_id)

    for document_id in deleted:
        logger.info("DELETED %s", document_id)

    embedded_nodes = [
        node for nodes in snapshot_nodes.values() if nodes for node in nodes
    ]
    if not embedded_nodes and not deleted:
        # every NEW/CHANGED doc failed to parse/embed (already logged)
        return 0

    version = registry.next_version()
    build = builder.build(
        version=version,
        registry=registry,
        embedder=embedder,
        nodes=embedded_nodes,
    )
    logger.info(
        "registration: embedding_model=%s dimension=%s",
        embedder.model,
        build.embedding_dimension,
    )

    for document_id in current_hashes:
        nodes = snapshot_nodes.get(document_id)
        if nodes is None:
            logger.error("Document %s skipped (not indexed in %s)", document_id, version)
            continue
        change = changes.get(document_id, Change.UNCHANGED)
        record = catalog.record_detection(
            document_id,
            change,
            current_hashes[document_id],
            detected_at=build.version,
        )
        record.chunk_ids = [node.id_ for node in nodes]
    for document_id in deleted:
        catalog.record_detection(
            document_id, Change.DELETED, "", detected_at=build.version
        )
    catalog.save(settings.document_catalog)
    logger.info("Index snapshot %s registered as current", build.version)
    return 0


def run_rollback(settings: Settings, version: str) -> int:
    try:
        Versioning(settings).rollback(version)
    except Exception as exc:
        logger.error("%s", exc)
        print(f"error: {exc}", file=sys.stderr)
        return 1
    print(f"rolled back to {version}")
    return 0


def run_versions(settings: Settings) -> int:
    registry = IndexRegistry().load(settings.index_registry)
    if not registry.versions:
        print("no index versions recorded")
        return 0
    print(f"current_version: {registry.current_version or '-'}")
    for version in sorted(registry.versions):
        config = registry.versions[version]
        print(
            f"  {version}  {config.collection_name}  "
            f"chunks={config.chunk_count}  docs={config.document_count}  "
            f"model={config.embedding_model}  dim={config.embedding_dimension}"
        )
    return 0


def run_catalog(settings: Settings) -> int:
    catalog = DocumentCatalog().load(settings.document_catalog)
    if not catalog.records:
        print("no documents in catalog")
        return 0
    for document_id, record in catalog.items():
        print(
            f"{document_id}  v{record.version[1:]}  hash={record.hash[:12]}  "
            f"source={record.source}  format={record.format}  "
            f"chunks={len(record.chunk_ids)}"
        )
    return 0


def run_search(settings: Settings, query: str, top_k: int | None = None) -> int:
    if not settings.openrouter_api_key:
        print("error: OPENROUTER_API_KEY not set (required to embed the query)", file=sys.stderr)
        return 1
    retriever = Retriever(settings, build_embedder(settings))
    try:
        results = retriever.retrieve(query, top_k=top_k)
    except RetrievalError as exc:
        print(f"error: {exc}", file=sys.stderr)
        logger.error("%s", exc)
        return 1
    print(f"top-{len(results)} for: {query}")
    for i, chunk in enumerate(results, start=1):
        print(f"\n[{i}] {chunk.chunk_id} (score={chunk.score})")
        print(f"    doc={chunk.lineage.get('document_id')} "
              f"version={chunk.lineage.get('version')} "
              f"source={chunk.lineage.get('source')} "
              f"index_version={chunk.lineage.get('index_version')}")
        print(f"    {chunk.text[:200]}")
    if not results:
        print("no results")
    return 0


def _parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="rag-dataops", description="RAG DataOps pipeline (LlamaIndex + ChromaDB)"
    )
    parser.add_argument("--verbose", action="store_true", help="debug logging")
    sub = parser.add_subparsers(dest="command", required=True)

    p_index = sub.add_parser(
        "index", help="discover, hash, change-detect, build snapshot, bump version"
    )
    p_index.set_defaults(func=run_index)

    p_rollback = sub.add_parser(
        "rollback", help="point current_version back to a stored snapshot"
    )
    p_rollback.add_argument("version")
    p_rollback.set_defaults(func=run_rollback)

    p_versions = sub.add_parser("versions", help="list index versions + configuration")
    p_versions.set_defaults(func=run_versions)
    sub.add_parser("catalog", help="show document catalog").set_defaults(
        func=run_catalog
    )

    p_search = sub.add_parser("search", help="top-k chunks with lineage metadata")
    p_search.add_argument("query")
    p_search.add_argument("--top-k", type=int, default=None)
    p_search.set_defaults(func=run_search)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = _parse_args(argv)
    settings = get_settings()
    configure_logging(settings, verbose=args.verbose)
    return args.func(settings, args)


if __name__ == "__main__":
    raise SystemExit(main())