"""CLI entrypoint and orchestration for the RAG indexing and retrieval pipeline."""

from __future__ import annotations

import argparse
import json
import logging
from collections.abc import Sequence

from rag.config import Settings
from rag.embed import Embedder, EmbeddingError, build_embedder
from rag.extract import ExtractionError, extract_pdf
from rag.generate import GeneratedAnswer, Generator, build_generator
from rag.index import Indexer, IndexOperationError
from rag.retrieve import RetrievalResult, Retriever
from rag.state import Change, detect_changes, hash_documents

logger = logging.getLogger(__name__)


def configure_logging(settings: Settings) -> None:
    """Route INFO+ logs to the console and to ``logs/indexing.log``."""
    settings.log_path.parent.mkdir(parents=True, exist_ok=True)
    root = logging.getLogger()
    root.setLevel(logging.INFO)
    if not any(
        isinstance(handler, logging.FileHandler)
        and getattr(handler, "baseFilename", None) == str(settings.log_path)
        for handler in root.handlers
    ):
        file_handler = logging.FileHandler(settings.log_path, encoding="utf-8")
        file_handler.setFormatter(logging.Formatter("%(asctime)s %(levelname)s %(message)s"))
        root.addHandler(file_handler)
    if not any(
        isinstance(handler, logging.StreamHandler) and not isinstance(handler, logging.FileHandler)
        for handler in root.handlers
    ):
        root.addHandler(logging.StreamHandler())


def run_index(settings: Settings, *, embedder: Embedder | None = None) -> Indexer:
    """Run incremental indexing: discover → detect → extract → clean → chunk → embed → update."""
    embedder = embedder or build_embedder(settings)
    indexer = Indexer(embedder, settings=settings)

    current_hashes = hash_documents(settings.data_dir)
    logger.info("Discovered %d PDF documents", len(current_hashes))
    changes = detect_changes(current_hashes, indexer.state)

    indexed = unchanged = deleted = failed = 0
    for relative_path in sorted(changes):
        change = changes[relative_path]

        if change is Change.UNCHANGED:
            logger.info("UNCHANGED %s; no processing required", relative_path)
            unchanged += 1
            continue

        if change is Change.DELETED:
            logger.info("DELETED %s", relative_path)
            try:
                indexer.remove_document(relative_path)
            except IndexOperationError as exc:
                logger.critical("Indexing aborted: failed to remove %s: %s", relative_path, exc)
                break
            deleted += 1
            logger.info("Removed %s", relative_path)
            continue

        logger.info("%s %s", change.value, relative_path)
        try:
            pdf_path = settings.data_dir / relative_path
            document = extract_pdf(pdf_path, document_id=relative_path)
            vector_ids = indexer.index_document(
                document, document_hash=current_hashes[relative_path]
            )
            logger.info("Indexed %s (%d vectors)", relative_path, len(vector_ids))
            indexed += 1
        except (ExtractionError, EmbeddingError) as exc:
            logger.error("%s processing failed for %s: %s", change.value, relative_path, exc)
            failed += 1
        except IndexOperationError as exc:
            logger.critical("Indexing aborted for %s: %s", relative_path, exc)
            break
        except Exception as exc:
            logger.critical("Unexpected error aborting indexing for %s: %s", relative_path, exc)
            break

    logger.info(
        "Indexing completed: %d indexed, %d unchanged, %d deleted, %d failed",
        indexed,
        unchanged,
        deleted,
        failed,
    )
    return indexer


def run_search(
    settings: Settings,
    question: str,
    *,
    embedder: Embedder | None = None,
    top_k: int | None = None,
) -> list[RetrievalResult]:
    """Embed ``question`` and retrieve the top-k matching chunks."""
    embedder = embedder or build_embedder(settings)
    retriever = Retriever(embedder, settings=settings)
    results = retriever.search(question, top_k=top_k)
    logger.info("Retrieved %d results for question", len(results))
    return results


def run_ask(
    settings: Settings,
    question: str,
    *,
    embedder: Embedder | None = None,
    generator: Generator | None = None,
    top_k: int | None = None,
) -> GeneratedAnswer:
    """Retrieve context for ``question`` and generate an answer with sources."""
    results = run_search(settings, question, embedder=embedder, top_k=top_k)
    generator = generator or build_generator(settings)
    answer = generator.generate(question, results)
    logger.info("Generated answer from %d sources", len(answer.sources))
    return answer


def run_reindex(settings: Settings, *, embedder: Embedder | None = None) -> Indexer:
    """Rebuild the index from scratch, re-embedding every document."""
    for path in (
        settings.index_path,
        settings.metadata_path,
        settings.document_state_path,
        settings.manifest_path,
    ):
        if path.exists():
            path.unlink()
    return run_index(settings, embedder=embedder)


def main(argv: Sequence[str] | None = None, *, settings: Settings | None = None) -> int:
    """Parse CLI arguments and dispatch to the matching pipeline stage."""
    settings = settings or Settings()
    parser = argparse.ArgumentParser(
        prog="rag-indexer", description="RAG indexing and retrieval pipeline"
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("index", help="Run incremental indexing")
    subparsers.add_parser("reindex", help="Rebuild the index from scratch")

    search_parser = subparsers.add_parser("search", help="Retrieve matching chunks")
    search_parser.add_argument("question")
    search_parser.add_argument("--top-k", type=int, default=None)

    ask_parser = subparsers.add_parser("ask", help="Retrieve context and generate an answer")
    ask_parser.add_argument("question")
    ask_parser.add_argument("--top-k", type=int, default=None)

    args = parser.parse_args(argv)

    if args.command == "index":
        run_index(settings)
    elif args.command == "reindex":
        run_reindex(settings)
    elif args.command == "search":
        for result in run_search(settings, args.question, top_k=args.top_k):
            print(json.dumps(result.model_dump(mode="json")))
    elif args.command == "ask":
        answer = run_ask(settings, args.question, top_k=args.top_k)
        print(answer.answer)
        for source in answer.sources:
            print(json.dumps(source.model_dump(mode="json")))
    return 0


if __name__ == "__main__":
    configure_logging(Settings())
    raise SystemExit(main())
