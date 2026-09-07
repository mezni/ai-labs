"""Unit tests for retrieval and generation evaluation modules."""

from __future__ import annotations

import json

import pytest

from src.evaluation import retrieval as eval_retrieval
from src.evaluation.generation import extract_json, judge_answer
from src.evaluation.metrics import (
    EvaluationError,
    cosine_similarity,
    hallucination_ratio,
    hit_at_k,
    hit_rate,
    lexical_groundedness,
    mrr,
    precision_at_k,
    recall_at_k,
    reciprocal_rank,
    reciprocal_rank_at_k,
)

CHUNKS = [
    {"chunk_id": "c1", "source": "a.txt"},
    {"chunk_id": "c2", "source": "b.txt"},
    {"chunk_id": "c3", "source": "a.txt"},
    {"chunk_id": "c4", "source": "b.txt"},
]
RELEVANT_A = {"c1", "c3"}
RETRIEVED = ["c1", "c2", "c4", "c3"]


def test_expected_source_chunks():
    assert eval_retrieval.expected_source_chunks(CHUNKS, "a.txt") == {"c1", "c3"}
    assert eval_retrieval.expected_source_chunks(CHUNKS, "missing.txt") == set()


@pytest.mark.parametrize("k", [0, -1])
def test_metrics_reject_non_positive_k(k):
    for fn in (recall_at_k, precision_at_k, hit_at_k, reciprocal_rank_at_k):
        with pytest.raises(EvaluationError):
            fn(RELEVANT_A, RETRIEVED, k)


def test_hit_at_k():
    assert hit_at_k(RELEVANT_A, RETRIEVED, 1) == 1.0
    assert hit_at_k({"c4"}, RETRIEVED, 1) == 0.0
    assert hit_at_k({"c4"}, RETRIEVED, 3) == 1.0
    assert hit_at_k(set(), RETRIEVED, 3) == 0.0


def test_recall_at_k():
    assert recall_at_k(RELEVANT_A, RETRIEVED, 1) == pytest.approx(1 / 2)
    assert recall_at_k(RELEVANT_A, ["c2", "c4"], 2) == 0.0
    assert recall_at_k(RELEVANT_A, ["c1", "c2", "c3"], 3) == 1.0
    assert recall_at_k(set(), RETRIEVED, 3) == 0.0


def test_precision_at_k():
    assert precision_at_k(RELEVANT_A, RETRIEVED, 1) == pytest.approx(1.0)
    assert precision_at_k(RELEVANT_A, ["c2", "c3"], 2) == pytest.approx(0.5)
    assert precision_at_k(RELEVANT_A, ["c2", "c4"], 2) == 0.0


def test_reciprocal_rank():
    assert reciprocal_rank(RELEVANT_A, RETRIEVED) == pytest.approx(1.0)
    assert reciprocal_rank(RELEVANT_A, ["c2", "c4", "c1"]) == pytest.approx(1 / 3)
    assert reciprocal_rank(RELEVANT_A, ["c5"]) == 0.0
    assert reciprocal_rank_at_k(RELEVANT_A, ["c2", "c4", "c1"], 2) == 0.0


def test_mrr_and_hit_rate():
    assert mrr([1.0, 0.5, 0.0]) == pytest.approx(0.5)
    assert mrr([]) == 0.0
    assert hit_rate([1.0, 1.0, 0.0]) == pytest.approx(2 / 3)


def test_cosine_similarity():
    assert cosine_similarity([1, 0], [1, 0]) == pytest.approx(1.0)
    assert cosine_similarity([1, 0], [0, 1]) == pytest.approx(0.0)
    assert cosine_similarity([], [1, 0]) == 0.0
    assert cosine_similarity([0, 0], [0, 0]) == 0.0


def test_lexical_groundedness():
    context = "the annual leave policy allows thirty days of leave"
    assert lexical_groundedness("Leave is thirty days.", context) > 0.5
    assert lexical_groundedness("Quantum teleportation is instant.", context) < 0.2


def test_hallucination_ratio():
    context = "the annual leave policy allows thirty days of leave"
    assert hallucination_ratio("Leave is thirty days.", context) < 0.3
    assert hallucination_ratio("Pink elephants on the moon.", context) > 0.5


class FakeRetriever:
    def __init__(self, chunks, order):
        self.kb = type("KB", (), {"chunks": chunks})()
        self._order = order

    def retrieve(self, query, top_k):
        return [{"chunk_id": cid} for cid in self._order.get(query, [])[:top_k]]


def test_evaluate_retrieval_aggregates():
    retriever = FakeRetriever(
        CHUNKS,
        {
            "q a": ["c1", "c2", "c3"],
            "q b": ["c4", "c3", "c5"],
        },
    )
    questions = [
        {"question": "q a", "expected_source": "a.txt"},
        {"question": "q b", "expected_source": "b.txt"},
    ]
    result = eval_retrieval.evaluate_retrieval(retriever, questions, top_k=3)
    assert result["num_questions"] == 2
    assert result["mrr"] == pytest.approx(1.0)
    assert result["hit_rate"] == 1.0
    assert result["mean_recall_at_k"] == pytest.approx(0.75)  # 2/2 + 1/2


def test_evaluate_retrieval_missing_expected_source():
    retriever = FakeRetriever(CHUNKS, {"q": ["c1"]})
    questions = [{"question": "q", "expected_source": "missing.txt"}]
    result = eval_retrieval.evaluate_retrieval(retriever, questions, top_k=3)
    assert result["per_question"][0]["recall_at_k"] == 0.0


def test_extract_json():
    assert extract_json('{"ok": true}') == {"ok": True}
    assert extract_json('here: {"ok": false} thanks') == {"ok": False}
    with pytest.raises(ValueError):
        extract_json("no json here")


def test_judge_answer_builds_messages_and_parses(tmp_path, monkeypatch):
    prompts_file = tmp_path / "prompts.yaml"
    prompts_file.write_text(
        """\
prompts:
  generation_eval:
    default_version: v1
    user_template: |
      QUESTION: {question}
      ANSWER: {answer}
    versions:
      v1:
        system: Return ONLY JSON.
""",
        encoding="utf-8",
    )

    class FakeClient:
        def generate(self, messages):
            assert messages[-1]["content"] == "QUESTION: q\nANSWER: the answer"
            return '{"correctness": 0.9, "grounded": true, "hallucinated": false}'

    class FakePromptManager:
        def get_prompt(self, name, version=None):
            return {"user_template": "QUESTION: {question}\nANSWER: {answer}", "system": "Return ONLY JSON."}

        def format_prompt(self, template, **kwargs):
            return template.format(**kwargs)

    verdict = judge_answer(
        FakeClient(), FakePromptManager(), "q", "the answer", "ref", "context text"
    )
    assert verdict == {"correctness": 0.9, "grounded": True, "hallucinated": False}


def test_evaluate_generation_aggregates_and_judge():
    class FakeEmbedder:
        def embed_documents(self, texts):
            # leave-topic -> [1, 0]; anything else -> [0, 1]
            return [[1.0, 0.0] if "leave" in t.lower() else [0.0, 1.0] for t in texts]

    class FakeLLMClient:
        def generate(self, messages):
            return '{"correctness": 1.0, "grounded": true, "hallucinated": false}'

    items = [
        {
            "question": "q1",
            "answer": "Leave policy is thirty days.",
            "expected_answer": "leave policy is thirty days",
            "context_chunks": [{"source": "a.md", "content": "leave policy thirty days"}],
        },
        {
            "question": "q2",
            "answer": "Unicorns are real.",
            "expected_answer": "leave policy is thirty days",
            "context_chunks": [{"source": "b.md", "content": "leave policy thirty days"}],
        },
    ]

    # -- lexical/embedding only --
    from src.evaluation.generation import evaluate_generation

    agg = evaluate_generation(items, FakeEmbedder())
    assert agg["num_questions"] == 2
    # q2 has no grounding -> hallucination ratio high, similarity low
    assert agg["hallucination_ratio"] >= 0.5
    assert agg["mean_similarity"] < 1.0

    # -- with LLM judge --
    agg_judged = evaluate_generation(
        items, FakeEmbedder(), use_llm_judge=True, client=FakeLLMClient(),
        prompt_manager=__import__("src.llm.prompt_manager", fromlist=["PromptManager"]).PromptManager(),
    )
    assert "correctness_judge" in agg_judged
    assert agg_judged["correctness_judge"] == 1.0
    assert agg_judged["hallucination_rate_judge"] == 0.0