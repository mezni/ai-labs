"""RAG evaluation: retrieval metrics and generation quality scoring."""
from src.evaluation.dataset import EVAL_QUESTIONS, load_dataset
from src.evaluation.metrics import (
    hit_at_k,
    precision_at_k,
    recall_at_k,
    reciprocal_rank,
)
from src.evaluation.retrieval import evaluate_retrieval
from src.evaluation.text import extract_json

__all__ = [
    "EVAL_QUESTIONS",
    "load_dataset",
    "hit_at_k",
    "precision_at_k",
    "recall_at_k",
    "reciprocal_rank",
    "evaluate_retrieval",
    "extract_json",
]