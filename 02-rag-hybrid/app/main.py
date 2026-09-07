"""RAG hybrid pipeline entry point.

Loads documents, builds the knowledge base index, and provides
a query interface combining BM25 (sparse) and vector (dense) retrieval
with LLM generation.
"""

from __future__ import annotations

import sys
from pathlib import Path

from src.knowledge.embeddings import EmbeddingGenerator
from src.knowledge.knowledge_base import Chunk, KnowledgeBase
from src.knowledge.retriever import Retriever
from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.utils.logger import get_logger

logger = get_logger(__name__)

DEFAULT_CORPUS_DIR = Path("data/policies")
DEFAULT_PERSIST_DIR = Path("data/chroma")


def build_knowledge_base(
    corpus_dir: str | Path = DEFAULT_CORPUS_DIR,
    persist_dir: str | Path = DEFAULT_PERSIST_DIR,
    reset: bool = False,
) -> tuple[KnowledgeBase, Retriever]:
    """Load documents, build the index, and return a ready-to-use KnowledgeBase and Retriever."""
    logger.info("Building knowledge base from %s", corpus_dir)
    embedder = EmbeddingGenerator()
    kb = KnowledgeBase(
        corpus_dir=corpus_dir,
        embedder=embedder,
        persist_dir=persist_dir,
    )
    if reset:
        kb.reset_collection()

    documents = kb.load_documents()
    kb.build_index(documents)

    retriever = Retriever(
        collection=kb.collection,
        embedder=embedder,
        chunks=kb.chunks,
    )
    logger.info(
        "Indexed %d chunks across %d documents into %r",
        len(kb.chunks),
        len(documents),
        kb.collection_name,
    )
    return kb, retriever


def query(
    question: str,
    retriever: Retriever,
    top_k: int = 5,
) -> list[Chunk]:
    """Retrieve the top-k relevant chunks for a question."""
    results = retriever.retrieve(question, top_k=top_k)
    for hit in results:
        logger.debug("  - %s (%.4f) [%s]", hit["chunk_id"], hit["score"], hit["source"])
    return results


def answer(
    question: str,
    retriever: Retriever,
    llm_client: LLMClient,
    prompt_manager: PromptManager,
    top_k: int = 5,
    prompt_version: str | None = None,
) -> str:
    """Run the full RAG pipeline: retrieve context and generate a grounded answer."""
    context = retriever.retrieve(question, top_k=top_k)
    logger.info("Retrieved %d context chunks for answer generation", len(context))
    messages = prompt_manager.build_generation_prompt(
        question, context, version=prompt_version
    )
    return llm_client.generate(messages)


def _parse_args(argv: list[str]) -> tuple[bool, str, str | None]:
    """Split CLI args into reset flag, question text, and optional prompt version."""
    reset = False
    version: str | None = None
    question_parts: list[str] = []
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg == "--reset":
            reset = True
        elif arg == "--prompt-version":
            if i + 1 < len(argv):
                version = argv[i + 1]
                i += 1
        else:
            question_parts.append(arg)
        i += 1
    question = " ".join(question_parts).strip()
    return reset, question, version


def main() -> None:
    """CLI entry point for building the index and answering a question."""
    reset, question, prompt_version = _parse_args(sys.argv[1:])

    kb, retriever = build_knowledge_base(reset=reset)

    if not question:
        logger.info("No question provided; index build complete.")
        return

    print(f"\nQuery: {question}\n")
    hits = query(question, retriever, top_k=5)
    for i, hit in enumerate(hits, 1):
        print(f"--- Result {i} (score: {hit['score']:.4f}) ---")
        print(f"Source: {hit['source']}")
        print(f"Content: {hit['content'][:200]}...")
        print()

    logger.info("Generating answer ...")
    with LLMClient() as llm_client:
        prompt_manager = PromptManager()
        print(
            answer(
                question,
                retriever,
                llm_client,
                prompt_manager,
                prompt_version=prompt_version,
            )
        )


if __name__ == "__main__":
    main()