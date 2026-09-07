"""Ranking metrics for retrieval evaluation."""
from __future__ import annotations


def precision_at_k(relevant: set[str], retrieved: list[str], k: int) -> float:
    """Fraction of the top-k retrieved chunk ids that are relevant."""
    if k <= 0:
        raise ValueError("k must be a positive integer")
    denominator = min(k, len(retrieved))
    if denominator == 0:
        return 0.0
    return len(relevant.intersection(retrieved[:k])) / denominator


def recall_at_k(relevant: set[str], retrieved: list[str], k: int) -> float:
    """Fraction of all relevant chunk ids covered by the top-k results."""
    if k <= 0:
        raise ValueError("k must be a positive integer")
    if not relevant:
        return 0.0
    return len(relevant.intersection(retrieved[:k])) / len(relevant)


def hit_at_k(relevant: set[str], retrieved: list[str], k: int) -> float:
    """1.0 if any relevant chunk appears in the top-k results, else 0.0."""
    if k <= 0:
        raise ValueError("k must be a positive integer")
    return 1.0 if any(chunk_id in relevant for chunk_id in retrieved[:k]) else 0.0


def reciprocal_rank(
    relevant: set[str], retrieved: list[str], k: int | None = None
) -> float:
    """1/rank of the first relevant chunk in the results (0.0 if none)."""
    if k is not None and k <= 0:
        raise ValueError("k must be a positive integer")
    subset = retrieved[:k] if k is not None else retrieved
    for rank, chunk_id in enumerate(subset, start=1):
        if chunk_id in relevant:
            return 1.0 / rank
    return 0.0