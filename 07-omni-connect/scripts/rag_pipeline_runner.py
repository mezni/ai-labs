"""
RAG Pipeline Runner
Orchestrates the omni-connect local RAG pipeline end-to-end over the
Category B knowledge base (data/knowledge_base/):

    build  ->  load/chunk the corpus, build the dense FAISS index
               (BAAI/bge-small-en-v1.5 via EmbeddingGenerator), rebuild the
               BM25 sparse index, then persist both to disk
    query  ->  load a prebuilt index and run a natural-language question
               through the hybrid Retriever (BM25 + FAISS candidates merged,
               then re-scored by BAAI/bge-reranker-base)

Retrieval behaviour (method, top-k, corpus glob/path) is read from
config/llm_config.yaml by the src.knowledge modules, so nothing is hard-coded
here beyond the index output location.

Index artifacts are written next to the corpus as:
    data/kb_store/
        - vector_index.index   (FAISS IndexFlatL2)
        - vector_index.docs    (pickled chunk metadata + content)

Run:  python scripts/rag_pipeline_runner.py build [--query "question"]
      python scripts/rag_pipeline_runner.py query "question" [--top-k N]
"""

import argparse
import sys
from pathlib import Path

# ==============================================================================
# 0. PATH WIRING & CONFIG
# ==============================================================================

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from src.knowledge.knowledge_base import KnowledgeBase  # noqa: E402
from src.knowledge.retriever import Retriever  # noqa: E402
from src.utils.config_loader import load_yaml_config  # noqa: E402
from src.utils.logger import setup_logger  # noqa: E402

logger = setup_logger(__name__)

CONFIG = load_yaml_config("config/llm_config.yaml")

CORPUS_PATH = Path(
    CONFIG.get("retrieval", {})
    .get("corpus", {})
    .get("path", "data/knowledge_base")
)
INDEX_STEM = ROOT / "data" / "kb_store" / "vector_index"


# ==============================================================================
# 1. QUERY PRINTING
# ==============================================================================


def print_hits(query: str, hits) -> None:
    """Render retrieval hits as a compact terminal report."""
    print(f'\nHits for "{query}"')
    print("-" * 72)
    if not hits:
        print("No hits returned.")
        return
    for rank, hit in enumerate(hits, start=1):
        source = hit["source"]
        chunk = hit.get("chunk_id", "?")
        snippet = " ".join(hit["content"].split())[:200]
        print(f"[{rank}] score={hit['score']:.4f}  {source} (chunk {chunk})")
        print(f"    {snippet}..." if len(snippet) == 200 else f"    {snippet}")
    print()


def run_query(kb: KnowledgeBase, query: str, top_k: int) -> None:
    """Instantiate the hybrid retriever over `kb` and run one question."""
    retriever = Retriever(kb)  # builds the sparse BM25 index for non-embedding methods
    hits = retriever.retrieve(query, top_k=top_k)
    print_hits(query, hits)


# ==============================================================================
# 2. COMMANDS
# ==============================================================================


def cmd_build(args: argparse.Namespace) -> int:
    """Load corpus -> chunk -> embed -> build FAISS + BM25 -> persist index."""
    kb = KnowledgeBase()

    documents = kb.load_documents(str(CORPUS_PATH))
    if not documents:
        logger.error("No corpus documents found under %s", CORPUS_PATH)
        return 1

    logger.info("Loaded %d chunks from %s", len(documents), CORPUS_PATH)

    kb.build_index(documents)
    kb.save_index(str(INDEX_STEM))
    logger.info("Persisted FAISS index + document store to %s", INDEX_STEM)

    # Dense index is saved; the sparse BM25 index is cheap and rebuilt on
    # demand by Retriever, but validate the full pipeline before exiting.
    if args.query:
        run_query(kb, args.query, args.top_k)
    else:
        run_query(kb, "Check trade-in rules for returning devices", args.top_k)

    return 0


def cmd_query(args: argparse.Namespace) -> int:
    """Load a prebuilt index and answer `args.query`."""
    if not (Path(f"{INDEX_STEM}.index").exists() and Path(f"{INDEX_STEM}.docs").exists()):
        logger.warning("Index not found at %s — building it first", INDEX_STEM)
        # Build in memory (no validation query) then continue to the user question.
        build_args = argparse.Namespace(
            load_only=True, query=None, top_k=args.top_k, question=args.question
        )
        cmd_build(build_args)

    kb = KnowledgeBase()
    kb.load_index(str(INDEX_STEM))
    logger.info("Loaded index (%d documents) from %s", len(kb.documents), INDEX_STEM)

    run_query(kb, args.question, args.top_k)
    return 0


# ==============================================================================
# 3. CLI ENTRYPOINT
# ==============================================================================


def main() -> int:
    parser = argparse.ArgumentParser(
        prog="rag_pipeline_runner",
        description="Run the omni-connect RAG pipeline: build the knowledge-base "
                    "index, then query it with the hybrid retriever.",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    build_parser = subparsers.add_parser(
        "build",
        help="chunk the corpus, build FAISS + BM25 indices, persist and validate",
    )
    build_parser.add_argument(
        "--query",
        nargs="?",
        const="Check trade-in rules for returning devices",
        default=None,
        help="run a validation query after building (default: a trade-in probe)",
    )
    build_parser.add_argument("--top-k", type=int, default=5)

    query_parser = subparsers.add_parser(
        "query", help='load a prebuilt index and answer "question"'
    )
    query_parser.add_argument("question", help="natural-language question to retrieve against")
    query_parser.add_argument("--top-k", type=int, default=5)

    args = parser.parse_args()

    if args.command == "build":
        return cmd_build(args)
    return cmd_query(args)


if __name__ == "__main__":
    raise SystemExit(main())