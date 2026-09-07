"""Retrieval evaluation orchestration: did we retrieve the right chunks?"""

from __future__ import annotations

from typing import Any

from src.evaluation import metrics
from src.knowledge.retriever import Retriever
from src.utils import get_logger

logger = get_logger(__name__)


def expected_source_chunks(chunks: list[dict[str, Any]], source: str) -> set[str]:
    """Chunk ids in the collection whose source matches the expected source."""
    return {c["chunk_id"] for c in chunks if c.get("source") == source}


def evaluate_retrieval(
    retriever: Retriever,
    questions: list[dict[str, str]],
    top_k: int = 5,
) -> dict[str, Any]:
    """Evaluate retrieval against per-question expected sources.

    Returns aggregated metrics plus a per-question breakdown. A question is
    ``relevant`` if any of its top-k chunks share the source expected in the
    dataset.
    """
    collection = retriever.kb.chunks
    rows: list[dict[str, Any]] = []
    for i, question in enumerate(questions, start=1):
        query = question["question"]
        expected = question.get("expected_source", "")
        relevant = expected_source_chunks(collection, expected)

        retrieved = retriever.retrieve(query, top_k=top_k)
        retrieved_ids = [r["chunk_id"] for r in retrieved]
        retrieved_sources = [r.get("source", "") for r in retrieved]

        if not relevant:
            logger.warning(
                "Q%d: expected source %r has no chunks in the collection", i, expected
            )
        rows.append(
            {
                "index": i,
                "question": query,
                "expected_source": expected,
                "retrieved_sources": retrieved_sources,
                "recall_at_k": metrics.recall_at_k(relevant, retrieved_ids, top_k),
                "precision_at_k": metrics.precision_at_k(relevant, retrieved_ids, top_k),
                "hit_at_k": metrics.hit_at_k(relevant, retrieved_ids, top_k),
                "reciprocal_rank": metrics.reciprocal_rank(relevant, retrieved_ids),
            }
        )

    return {
        "top_k": top_k,
        "num_questions": len(rows),
        "mrr": metrics.mrr([r["reciprocal_rank"] for r in rows]),
        "hit_rate": metrics.hit_rate([r["hit_at_k"] for r in rows]),
        "mean_recall_at_k": metrics.mean([r["recall_at_k"] for r in rows]),
        "mean_precision_at_k": metrics.mean([r["precision_at_k"] for r in rows]),
        "per_question": rows,
    }