"""Retrieval evaluation: how well does the retriever surface the right chunks?"""
from __future__ import annotations

from typing import TYPE_CHECKING, Any

from src.evaluation import metrics as m

if TYPE_CHECKING:  # pragma: no cover
    from src.knowledge.retriever import Retriever


def relevant_chunk_ids(chunks: list[dict[str, Any]], expected_source: str) -> set[str]:
    """Chunk ids that belong to the expected source document."""
    return {
        chunk["chunk_id"]
        for chunk in chunks
        if chunk.get("source") == expected_source
    }


def evaluate_retrieval(
    retriever: "Retriever",
    chunks: list[dict[str, Any]],
    dataset: list[dict[str, Any]],
    k_values: tuple[int, ...] = (1, 3, 5),
) -> dict[str, Any]:
    """Score every question in the dataset against the retriever.

    `dataset` entries are expected to have "question" and "expected_source" keys.
    Relevancy is defined by source document: a chunk is relevant when its
    `source` matches the entry's `expected_source`.
    """
    per_query: list[dict[str, Any]] = []
    for entry in dataset:
        question = entry["question"]
        expected_source = entry["expected_source"]
        relevant = relevant_chunk_ids(chunks, expected_source)
        row: dict[str, Any] = {
            "question": question,
            "expected_source": expected_source,
            "num_relevant": len(relevant),
        }
        for k in k_values:
            retrieved = retriever.retrieve(question, top_k=k)
            retrieved_ids = [chunk["chunk_id"] for chunk in retrieved]
            row[f"hit@{k}"] = m.hit_at_k(relevant, retrieved_ids, k)
            row[f"recall@{k}"] = m.recall_at_k(relevant, retrieved_ids, k)
            row[f"precision@{k}"] = m.precision_at_k(relevant, retrieved_ids, k)
            row[f"mrr@{k}"] = m.reciprocal_rank(relevant, retrieved_ids, k)
        per_query.append(row)

    def _mean(field: str) -> dict[int, float]:
        return {
            k: sum(row[f"{field}@{k}"] for row in per_query) / len(per_query)
            for k in k_values
        }

    return {
        "k_values": list(k_values),
        "num_queries": len(per_query),
        "metrics": {
            "hit_rate": _mean("hit"),
            "recall_at_k": _mean("recall"),
            "precision_at_k": _mean("precision"),
            "mrr": _mean("mrr"),
        },
        "per_query": per_query,
    }