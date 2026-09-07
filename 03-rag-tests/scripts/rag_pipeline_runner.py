"""RAG pipeline runner.

End-to-end script: index the knowledge base, retrieve relevant chunks for a
query, format the retrieval prompt, and generate a grounded answer.

Prerequisites:
    - OPENROUTER_API_KEY set (chat + embeddings)
    - a corpus of markdown documents (default: data/policies, produced by
      scripts/generate_docs.py)

Run:
    uv run python scripts/rag_pipeline_runner.py --query "How much is plan upgrade?"
    uv run python scripts/rag_pipeline_runner.py --query "..." --rerank --rebuild
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from src.knowledge import Reranker
from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever
from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.observability import RequestLogger
from src.utils import get_logger

logger = get_logger(__name__)


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run the RAG pipeline end to end.")
    parser.add_argument("--query", "-q", required=True, help="User question to answer.")
    parser.add_argument("--top-k", type=int, default=5, help="Final context chunks.")
    parser.add_argument("--sparse-top-k", type=int, default=15, help="BM25 candidates.")
    parser.add_argument("--dense-top-k", type=int, default=15, help="FAISS candidates.")
    parser.add_argument(
        "--prompt-version",
        default=None,
        help="Prompt version to use (default: configured default_version).",
    )
    parser.add_argument(
        "--rerank", action="store_true", help="Re-score fused candidates with the reranker."
    )
    parser.add_argument(
        "--rebuild",
        action="store_true",
        help="Force a rebuild of the index instead of loading a cached one.",
    )
    parser.add_argument("--corpus-dir", type=Path, default=Path("data/policies"))
    parser.add_argument("--index-path", type=Path, default=Path("data/faiss/index.bin"))
    parser.add_argument("--chunks-path", type=Path, default=Path("data/faiss/chunks.json"))
    return parser.parse_args()


def build_knowledge_base(args: argparse.Namespace) -> KnowledgeBase:
    """Load cached index, or rebuild it from the corpus when needed."""
    kb = KnowledgeBase(
        corpus_dir=args.corpus_dir,
        index_path=args.index_path,
        chunks_path=args.chunks_path,
    )
    cached = args.index_path.exists() and args.chunks_path.exists()
    if cached and not args.rebuild:
        logger.info("Loading cached index from %s", args.index_path)
        kb.load_index()
        return kb

    logger.info("Building index from %s", args.corpus_dir)
    documents = kb.load_documents()
    if not documents:
        raise SystemExit(
            f"No markdown documents found in {args.corpus_dir.resolve()}; "
            "run scripts/generate_docs.py first."
        )
    kb.build_index(documents)
    kb.save_index()
    return kb


def retrieve_context(retriever: Retriever, query: str, top_k: int) -> list[dict]:
    """Retrieve context chunks for the query."""
    chunks = retriever.retrieve(query, top_k=top_k)
    logger.info("Retrieved %d chunks for %r", len(chunks), query)
    for chunk in chunks:
        logger.debug(
            "  [%.3f] %s (%s)", chunk["score"], chunk["source"], chunk["chunk_id"]
        )
    return chunks


def build_messages(prompt_manager: PromptManager, query: str, context: list[dict], version: str | None) -> list[dict[str, str]]:
    """Assemble the system + user messages from the retrieval prompt template."""
    prompt = prompt_manager.get_prompt("retrieval_query", version=version)
    system = prompt.get("system", "")
    user = prompt_manager.format_prompt(
        prompt["user_template"],
        context="\n\n".join(f"[{c['source']}]\n{c['content']}" for c in context),
        query=query,
    )
    messages: list[dict[str, str]] = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": user})
    return messages


def main() -> None:
    args = _parse_args()
    kb = build_knowledge_base(args)

    reranker = Reranker() if args.rerank else None
    retriever = Retriever(
        kb,
        sparse_top_k=args.sparse_top_k,
        dense_top_k=args.dense_top_k,
        reranker=reranker,
    )

    context = retrieve_context(retriever, args.query, args.top_k)
    if not context:
        raise SystemExit("No context retrieved for the query; cannot generate an answer.")

    with RequestLogger() as logger_rec:
        logger_rec.start(question=args.query)
        logger_rec.set_retrieval(context)

        prompt_manager = PromptManager()
        messages = build_messages(prompt_manager, args.query, context, args.prompt_version)
        llm_client = LLMClient()
        logger_rec.set_prompt(messages)
        logger_rec.set_model(llm_client.model)

        try:
            answer, usage = llm_client.generate_with_usage(messages)
            logger_rec.finish(
                answer=answer,
                usage=usage,
                sources=sorted({chunk["source"] for chunk in context}),
            )
        except Exception as exc:
            logger_rec.record_error(exc)
            logger_rec.finish(answer="", sources=sorted({c["source"] for c in context}))
            raise
        finally:
            llm_client.close()

        print("\n" + "=" * 70)
        print(f"Q: {args.query}")
        print("-" * 70)
        print(answer)
        print("-" * 70)
        print("Sources:")
        for chunk in context:
            print(f"  - {chunk['source']} (score={chunk['score']:.3f})")
    kb.close()


if __name__ == "__main__":
    main()