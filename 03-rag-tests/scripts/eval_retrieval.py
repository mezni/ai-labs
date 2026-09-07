"""Retrieval evaluation runner.

Measures whether the right chunks were retrieved for each question in the
evaluation dataset: Recall@K, Precision@K, MRR, and Hit Rate.

Run:
    uv run python scripts/eval_retrieval.py
    uv run python scripts/eval_retrieval.py --ks 1,3,5 --json-out results/retrieval.json
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from src.evaluation import retrieval as eval_retrieval
from src.knowledge import Reranker
from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever
from src.utils import get_logger

logger = get_logger(__name__)

DEFAULT_QUESTIONS = Path("data/validation/eval_questions.json")


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Evaluate RAG retrieval quality.")
    parser.add_argument("--questions", type=Path, default=DEFAULT_QUESTIONS)
    parser.add_argument(
        "--ks", default="1,3,5", help="Comma-separated top-K values to evaluate."
    )
    parser.add_argument("--sparse-top-k", type=int, default=15)
    parser.add_argument("--dense-top-k", type=int, default=15)
    parser.add_argument("--rerank", action="store_true")
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


def main() -> None:
    args = _parse_args()
    questions = json.loads(args.questions.read_text(encoding="utf-8"))
    retriever = load_retriever(args)
    ks = [int(k) for k in args.ks.split(",") if k.strip()]

    print(f"\nRetrieval evaluation over {len(questions)} questions")
    print(f"{'K':>3} | {'Recall@K':>9} | {'Precision@K':>11} | {'Hit@K':>6} | {'MRR@K':>6}")
    print("-" * 50)
    aggregates = {}
    for k in ks:
        result = eval_retrieval.evaluate_retrieval(retriever, questions, top_k=k)
        print(
            f"{k:>3} | {result['mean_recall_at_k']:>9.3f} | "
            f"{result['mean_precision_at_k']:>11.3f} | {result['hit_rate']:>6.3f} | "
            f"{result['mrr']:>6.3f}"
        )
        aggregates[k] = {
            "recall_at_k": result["mean_recall_at_k"],
            "precision_at_k": result["mean_precision_at_k"],
            "hit_at_k": result["hit_rate"],
            "mrr_at_k": result["mrr"],
        }

    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(
            json.dumps({"questions": aggregates, "per_question": result["per_question"]}, indent=2),
            encoding="utf-8",
        )
        print(f"\nSaved details to {args.json_out}")


if __name__ == "__main__":
    main()