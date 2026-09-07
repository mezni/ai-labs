"""Generation evaluation runner: correctness, groundedness, hallucination.

Runs the RAG pipeline for each question, then evaluates the produced answer:
  - Correctness: embedding similarity vs. the expected answer (+ LLM judge)
  - Groundedness: is the answer supported by the retrieved context?
  - Hallucination: does the answer state facts absent from the context?

Run:
    uv run python scripts/eval_generation.py
    uv run python scripts/eval_generation.py --judge --json-out results/generation.json
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from src.evaluation import generation as eval_generation
from src.knowledge import EmbeddingGenerator, Reranker
from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever
from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.utils import get_logger

logger = get_logger(__name__)

DEFAULT_QUESTIONS = Path("data/validation/eval_questions.json")


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Evaluate RAG answer quality.")
    parser.add_argument("--questions", type=Path, default=DEFAULT_QUESTIONS)
    parser.add_argument("--top-k", type=int, default=5, help="Context chunks.")
    parser.add_argument("--prompt-version", default=None)
    parser.add_argument("--sparse-top-k", type=int, default=15)
    parser.add_argument("--dense-top-k", type=int, default=15)
    parser.add_argument("--rerank", action="store_true")
    parser.add_argument("--judge", action="store_true", help="Use LLM-as-judge.")
    parser.add_argument("--similarity-threshold", type=float, default=0.70)
    parser.add_argument("--rebuild", action="store_true")
    parser.add_argument("--json-out", type=Path, default=None)
    return parser.parse_args()


def load_retriever(args: argparse.Namespace) -> Retriever:
    kb = KnowledgeBase()
    if args.rebuild or not (kb.index_path.exists() and kb.chunks_path.exists()):
        documents = kb.load_documents()
        kb.build_index(documents)
        kb.save_index()
    else:
        kb.load_index()
    return Retriever(
        kb,
        sparse_top_k=args.sparse_top_k,
        dense_top_k=args.dense_top_k,
        reranker=Reranker() if args.rerank else None,
    )


def generate_for_question(
    retriever: Retriever,
    prompt_manager: PromptManager,
    question: str,
    top_k: int,
    prompt_version: str | None,
) -> tuple[str, list[dict]]:
    """Retrieve context and generate an answer; return answer + context chunks."""
    chunks = retriever.retrieve(question, top_k=top_k)
    prompt = prompt_manager.get_prompt("retrieval_query", version=prompt_version)
    user = prompt_manager.format_prompt(
        prompt["user_template"],
        context="\n\n".join(f"[{c['source']}]\n{c['content']}" for c in chunks),
        query=question,
    )
    messages: list[dict[str, str]] = []
    if prompt.get("system"):
        messages.append({"role": "system", "content": prompt["system"]})
    messages.append({"role": "user", "content": user})
    answer = LLMClient().generate(messages)
    return answer, chunks


def main() -> None:
    args = _parse_args()
    questions = json.loads(args.questions.read_text(encoding="utf-8"))
    retriever = load_retriever(args)
    embedder = EmbeddingGenerator()
    prompt_manager = PromptManager()
    client = LLMClient() if args.judge else None

    items = []
    for question in questions:
        answer, chunks = generate_for_question(
            retriever, prompt_manager, question["question"], args.top_k, args.prompt_version
        )
        items.append(
            {
                "question": question["question"],
                "answer": answer,
                "expected_answer": question["expected_answer"],
                "context_chunks": chunks,
            }
        )
        print(f"Q: {question['question'][:60]:<62} -> answered")

    result = eval_generation.evaluate_generation(
        items,
        embedder,
        similarity_threshold=args.similarity_threshold,
        use_llm_judge=args.judge,
        client=client,
        prompt_manager=prompt_manager,
    )

    print(f"\nGeneration evaluation over {result['num_questions']} questions")
    print(f"  Correctness (lexical):   {result['correctness_lexical']:.3f}  "
          f"(mean similarity {result['mean_similarity']:.3f})")
    print(f"  Groundedness (lexical):  {result['groundedness']:.3f}")
    print(f"  Hallucination ratio:     {result['hallucination_ratio']:.3f}")
    if args.judge:
        print(f"  Correctness (judge):     {result['correctness_judge']:.3f}")
        print(f"  Grounded (judge):        {result['groundedness_judge']:.3f}")
        print(f"  Hallucination (judge):   {result['hallucination_rate_judge']:.3f}")

    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(
            json.dumps(result, indent=2, default=str), encoding="utf-8"
        )
        print(f"\nSaved details to {args.json_out}")


if __name__ == "__main__":
    main()