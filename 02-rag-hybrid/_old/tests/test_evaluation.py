"""Tests for RAG evaluation metrics and orchestration."""
import pytest

from src.evaluation.metrics import hit_at_k, precision_at_k, recall_at_k, reciprocal_rank
from src.evaluation.retrieval import evaluate_retrieval, relevant_chunk_ids
from src.evaluation.text import extract_json

CHUNKS = [
    {"chunk_id": "c1", "source": "a.txt"},
    {"chunk_id": "c2", "source": "b.txt"},
    {"chunk_id": "c3", "source": "a.txt"},
    {"chunk_id": "c4", "source": "b.txt"},
]
RELEVANT_A = {"c1", "c3"}
RETRIEVED = ["c1", "c2", "c4", "c3"]


def test_relevant_chunk_ids_filters_by_source():
    assert relevant_chunk_ids(CHUNKS, "a.txt") == {"c1", "c3"}
    assert relevant_chunk_ids(CHUNKS, "missing.txt") == set()


@pytest.mark.parametrize("k", [0, -1])
def test_metrics_reject_non_positive_k(k):
    with pytest.raises(ValueError):
        precision_at_k(RELEVANT_A, RETRIEVED, k)
    with pytest.raises(ValueError):
        recall_at_k(RELEVANT_A, RETRIEVED, k)
    with pytest.raises(ValueError):
        hit_at_k(RELEVANT_A, RETRIEVED, k)
    with pytest.raises(ValueError):
        reciprocal_rank(RELEVANT_A, RETRIEVED, k)


def test_hit_at_k():
    assert hit_at_k(RELEVANT_A, RETRIEVED, 1) == 1.0
    assert hit_at_k({"c4"}, RETRIEVED, 1) == 0.0
    assert hit_at_k({"c4"}, RETRIEVED, 3) == 1.0
    assert hit_at_k(set(), RETRIEVED, 3) == 0.0


def test_precision_and_recall_at_k():
    assert precision_at_k(RELEVANT_A, RETRIEVED, 3) == pytest.approx(1 / 3)
    assert recall_at_k(RELEVANT_A, RETRIEVED, 3) == pytest.approx(0.5)
    assert recall_at_k(RELEVANT_A, RETRIEVED, 4) == pytest.approx(1.0)
    assert recall_at_k(set(), RETRIEVED, 3) == 0.0


def test_reciprocal_rank():
    assert reciprocal_rank(RELEVANT_A, RETRIEVED) == pytest.approx(1.0)
    assert reciprocal_rank({"c4"}, RETRIEVED) == pytest.approx(1 / 3)
    assert reciprocal_rank({"c4"}, RETRIEVED, k=2) == 0.0
    assert reciprocal_rank({"c2"}, RETRIEVED) == pytest.approx(0.5)
    assert reciprocal_rank(set(), RETRIEVED) == 0.0


class FakeRetriever:
    def __init__(self, by_question: dict[str, list[str]]) -> None:
        self._by_question = by_question

    def retrieve(self, question: str, top_k: int = 5):
        result = []
        for chunk_id in self._by_question.get(question, [])[:top_k]:
            result.append({"chunk_id": chunk_id})
        return result


DATASET = [
    {"question": "q1", "expected_answer": "-", "expected_source": "a.txt"},
    {"question": "q2", "expected_answer": "-", "expected_source": "b.txt"},
    {"question": "q3", "expected_answer": "-", "expected_source": "missing.txt"},
]


def test_evaluate_retrieval_aggregates_metrics():
    retriever = FakeRetriever(
        {
            "q1": RETRIEVED,
            "q2": RETRIEVED,
            "q3": RETRIEVED,
        }
    )

    report = evaluate_retrieval(retriever, CHUNKS, DATASET, k_values=(3,))

    metrics = report["metrics"]
    assert report["num_queries"] == 3
    assert metrics["hit_rate"][3] == pytest.approx(2 / 3)
    assert metrics["recall_at_k"][3] == pytest.approx((0.5 + 1.0 + 0.0) / 3)
    assert metrics["precision_at_k"][3] == pytest.approx((1 / 3 + 2 / 3 + 0.0) / 3)
    assert metrics["mrr"][3] == pytest.approx((1.0 + 0.5 + 0.0) / 3)


def test_evaluate_retrieval_per_query_rows():
    report = evaluate_retrieval(FakeRetriever({"q1": RETRIEVED}), CHUNKS, DATASET[:1], k_values=(1, 3))

    row = report["per_query"][0]
    assert row["question"] == "q1"
    assert row["expected_source"] == "a.txt"
    assert row["num_relevant"] == 2
    assert row["hit@1"] == 1.0
    assert row["hit@3"] == 1.0


def test_extract_json_handles_prose_and_fences():
    assert extract_json("done\n```json\n{\"correctness_score\": 4}\n```") == {
        "correctness_score": 4
    }
    assert extract_json('prefix {"a": 1} suffix') == {"a": 1}
    assert extract_json("no json here") is None
    assert extract_json("") is None