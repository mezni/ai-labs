"""Evaluation CLI: build the pipeline, run retrieval + generation evaluation."""
from __future__ import annotations

import argparse
import json
import os
from typing import Any

from src.evaluation.dataset import load_dataset
from src.evaluation.generation import evaluate_generation
from src.evaluation.retrieval import evaluate_retrieval
from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever


def chunks_from_collection(collection: Any) -> list[dict[str, Any]]:
    """Rebuild chunk records from a persisted collection."""
    result = collection.get(include=["documents", "metadatas"])
    return [
        {
            "chunk_id": chunk_id,
            "content": document,
            "document_id": metadata.get("document_id", ""),
            "source": metadata.get("source", ""),
            "category": metadata.get("category", ""),
        }
        for chunk_id, document, metadata in zip(
            result["ids"],
            result["documents"],
            result["metadatas"],
        )
    ]


def run_evaluation(
    *,
    dataset_path: str | None = None,
    k_values: tuple[int, ...] = (1, 3, 5),
    top_k: int = 5,
    rebuild: bool = False,
    skip_generation: bool = False,
) -> dict[str, Any]:
    """Build the KB/retriever (and generator), then evaluate both stages."""
    dataset = load_dataset(dataset_path)

    kb = KnowledgeBase()
    if rebuild or kb.collection.count() == 0:
        documents = kb.load_documents()
        kb.build_index(documents)
        kb.save_index(str(kb.persist_dir))
    chunks = chunks_from_collection(kb.collection)

    retriever = Retriever(kb.collection, kb.embedder, chunks)
    report: dict[str, Any] = {
        "retrieval": evaluate_retrieval(retriever, chunks, dataset, k_values),
    }

    if skip_generation or not os.environ.get("OPENROUTER_API_KEY"):
        report["generation"] = {
            "skipped": True,
            "reason": "OPENROUTER_API_KEY is not set (or --skip-generation)",
        }
        return report

    from app.main import Generator

    generator = Generator()
    report["generation"] = evaluate_generation(
        retriever, generator, dataset, top_k=top_k
    )
    return report


def print_report(report: dict[str, Any]) -> None:
    retrieval = report["retrieval"]
    k_values = retrieval["k_values"]
    print("=== Retrieval Evaluation ===")
    print(
        f"Queries: {retrieval['num_queries']}   "
        f"K values: {'/'.join(str(k) for k in k_values)}"
    )
    for metric, by_k in retrieval["metrics"].items():
        formatted = "  ".join(
            f"{metric}@{k}: {by_k[k]:.3f}" for k in k_values
        )
        print(f"  {formatted}")

    print("\n  Per query:")
    for row in retrieval["per_query"]:
        hits = " ".join(
            f"h@{k}={'Y' if row[f'hit@{k}'] else 'N'}" for k in k_values
        )
        print(f"    [{row['expected_source']}] {row['question']}  ->  {hits}")

    generation = report.get("generation")
    if generation is None:
        return
    if generation.get("skipped"):
        print(f"\n=== Generation Evaluation (skipped: {generation['reason']}) ===")
        return
    print("\n=== Generation Evaluation ===")
    metrics = generation["metrics"]
    print(
        f"  mean_correctness: {metrics['mean_correctness']:.3f}   "
        f"mean_faithfulness: {metrics['mean_faithfulness']:.3f}   "
        f"hallucination_rate: {metrics['hallucination_rate']:.3f}"
    )
    print("\n  Per query:")
    for row in generation["per_query"]:
        print(
            f"    [{row['question'][:70]}]  "
            f"correct={row['correctness_score']:.2f} "
            f"faithful={row['faithfulness_score']:.2f} "
            f"hallucination={'YES' if row['hallucination'] else 'no'}"
        )
        print(f"      answer: {row['answer'][:140]}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Evaluate the hybrid RAG pipeline")
    parser.add_argument("--dataset", default=None, help="Path to a YAML dataset")
    parser.add_argument(
        "--k-values",
        default="1,3,5",
        help="Comma-separated K values for retrieval metrics",
    )
    parser.add_argument(
        "--top-k", type=int, default=5, help="Chunks passed to the generator"
    )
    parser.add_argument(
        "--rebuild",
        action="store_true",
        help="Rebuild the vector index before evaluating",
    )
    parser.add_argument(
        "--skip-generation", action="store_true", help="Only evaluate retrieval"
    )
    parser.add_argument(
        "--output", default=None, help="Write the full report to a JSON file"
    )
    args = parser.parse_args(argv)

    k_values = tuple(int(k) for k in args.k_values.split(","))
    report = run_evaluation(
        dataset_path=args.dataset,
        k_values=k_values,
        top_k=args.top_k,
        rebuild=args.rebuild,
        skip_generation=args.skip_generation,
    )
    print_report(report)
    if args.output:
        with open(args.output, "w", encoding="utf-8") as handle:
            json.dump(report, handle, indent=2)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())