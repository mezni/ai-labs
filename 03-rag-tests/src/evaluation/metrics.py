"""Ranking and answer-quality metrics for RAG evaluation."""

from __future__ import annotations

import re

import numpy as np


class EvaluationError(ValueError):
    """Raised for invalid metric inputs."""


def _validate_k(k: int) -> None:
    if k < 1:
        raise EvaluationError("k must be a positive integer")


def recall_at_k(relevant: set[str], retrieved: list[str], k: int) -> float:
    """Fraction of relevant chunks found in the top-k retrieved chunks."""
    _validate_k(k)
    if not relevant:
        return 0.0
    return len(relevant.intersection(retrieved[:k])) / len(relevant)


def precision_at_k(relevant: set[str], retrieved: list[str], k: int) -> float:
    """Fraction of top-k retrieved chunks that are relevant."""
    _validate_k(k)
    top_k = retrieved[:k]
    if not top_k:
        return 0.0
    return len(relevant.intersection(top_k)) / len(top_k)


def hit_at_k(relevant: set[str], retrieved: list[str], k: int) -> float:
    """1.0 if at least one relevant chunk appears in the top-k, else 0.0."""
    _validate_k(k)
    return 1.0 if any(c in relevant for c in retrieved[:k]) else 0.0


def reciprocal_rank(relevant: set[str], retrieved: list[str]) -> float:
    """1/rank of the first relevant chunk (0.0 if none is retrieved)."""
    for rank, chunk_id in enumerate(retrieved, start=1):
        if chunk_id in relevant:
            return 1.0 / rank
    return 0.0


def reciprocal_rank_at_k(relevant: set[str], retrieved: list[str], k: int) -> float:
    """Reciprocal rank, treating anything past top-k as a miss."""
    _validate_k(k)
    return reciprocal_rank(relevant, retrieved[:k])


def mrr(rows: list[float]) -> float:
    """Mean reciprocal rank over per-query reciprocal ranks."""
    return _mean(rows)


def hit_rate(rows: list[float]) -> float:
    """Fraction of queries with at least one relevant hit in the top-k."""
    return _mean(rows)


def _mean(values: list[float]) -> float:
    return sum(values) / len(values) if values else 0.0


def mean(values: list[float]) -> float:
    """Arithmetic mean of a list of scores."""
    return _mean(values)


# -- answer quality -----------------------------------------------------------


def cosine_similarity(a: list[float] | np.ndarray, b: list[float] | np.ndarray) -> float:
    """Cosine similarity between two embeddings."""
    va = np.asarray(a, dtype=np.float64)
    vb = np.asarray(b, dtype=np.float64)
    na, nb = np.linalg.norm(va), np.linalg.norm(vb)
    if na == 0 or nb == 0:
        return 0.0
    return float(np.dot(va, vb) / (na * nb))


def _tokens(text: str) -> set[str]:
    return set(re.findall(r"[a-z0-9]+", text.lower()))


def split_sentences(text: str) -> list[str]:
    """Split an answer into sentences (strips short/empty fragments)."""
    return [s.strip() for s in re.split(r"[.?!\n]", text) if len(s.strip()) > 3]


def sentence_lexical_support(sentence: str, context: str, min_overlap: float = 0.3) -> float:
    """How well a sentence is covered by context tokens (0..1)."""
    tokens = _tokens(sentence)
    if not tokens:
        return 1.0
    context_tokens = _tokens(context)
    covered = len(tokens.intersection(context_tokens)) / len(tokens)
    return covered if covered >= min_overlap else 0.0


def lexical_groundedness(answer: str, context: str, min_overlap: float = 0.3) -> float:
    """Mean lexical support of the answer's sentences in the context (0..1)."""
    sentences = split_sentences(answer)
    if not sentences:
        return 0.0
    return np.mean(
        [sentence_lexical_support(s, context, min_overlap) for s in sentences]
    ).item()


def hallucination_ratio(answer: str, context: str, min_overlap: float = 0.3) -> float:
    """Fraction of answer sentences without lexical support in context (0..1)."""
    sentences = split_sentences(answer)
    if not sentences:
        return 1.0
    unsupported = [
        1 - sentence_lexical_support(s, context, min_overlap) for s in sentences
    ]
    return np.mean(unsupported).item()