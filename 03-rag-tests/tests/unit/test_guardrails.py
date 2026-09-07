"""Unit tests for the RAG guardrails (src/guardrails.py)."""

from __future__ import annotations

import pytest

from src.guardrails import (
    NO_RELEVANT_MESSAGE,
    UNGROUNDED_MESSAGE,
    EmptyQuestionError,
    check_grounding,
    check_retrieval,
    validate_query,
)

CHUNKS = [
    {"chunk_id": "c1", "source": "a.txt", "content": "RAG grounds answers in retrieved context.", "score": 0.016, "dense_similarity": 0.72},
    {"chunk_id": "c2", "source": "b.txt", "content": "hello world", "score": 0.015, "dense_similarity": 0.55},
]


def _patch_config(monkeypatch, **overrides):
    config = {
        "guardrails": {
            "retrieval": {"min_similarity": 0.30},
            "generation": {"min_groundedness": 0.30},
        }
    }
    config["guardrails"]["retrieval"].update(overrides.get("retrieval", {}))
    config["guardrails"]["generation"].update(overrides.get("generation", {}))
    import src.guardrails as guardrails

    monkeypatch.setattr(guardrails, "load_config", lambda: config)
    return config


# -- input guardrail ----------------------------------------------------------


@pytest.mark.parametrize("bad", [None, "", "   ", "\n\t"])
def test_validate_query_rejects_empty(monkeypatch, bad):
    _patch_config(monkeypatch)
    with pytest.raises(EmptyQuestionError):
        validate_query(bad)


@pytest.mark.parametrize("good", ["What is RAG?", "  What is RAG?  ", "What\nis RAG?"])
def test_validate_query_strips_and_passes(monkeypatch, good):
    _patch_config(monkeypatch)
    assert validate_query(good) == good.strip()


# -- retrieval guardrail ------------------------------------------------------


def test_check_retrieval_passes_on_high_similarity(monkeypatch):
    _patch_config(monkeypatch)
    assert check_retrieval(CHUNKS) is None


def test_check_retrieval_rejects_low_similarity(monkeypatch):
    _patch_config(monkeypatch)
    chunks = [{"chunk_id": "c1", "source": "a.txt", "content": "x", "score": 0.01, "dense_similarity": 0.10}]
    assert check_retrieval(chunks) == NO_RELEVANT_MESSAGE


def test_check_retrieval_rejects_empty_list(monkeypatch):
    _patch_config(monkeypatch)
    assert check_retrieval([]) == NO_RELEVANT_MESSAGE


def test_check_retrieval_falls_back_to_score_key(monkeypatch):
    _patch_config(monkeypatch)
    chunks = [{"chunk_id": "c1", "score": 0.60}]
    assert check_retrieval(chunks) is None


def test_check_retrieval_uses_explicit_threshold(monkeypatch):
    _patch_config(monkeypatch)
    chunks = [{"chunk_id": "c1", "score": 0.60, "dense_similarity": 0.60}]
    assert check_retrieval(chunks, min_similarity=0.9) == NO_RELEVANT_MESSAGE
    assert check_retrieval(chunks, min_similarity=0.5) is None


def test_check_retrieval_missing_config_uses_defaults(monkeypatch):
    import src.guardrails as guardrails

    monkeypatch.setattr(guardrails, "load_config", lambda: {"models": {}})
    assert check_retrieval(CHUNKS) is None
    assert check_retrieval([{"chunk_id": "c1", "dense_similarity": 0.1}]) == NO_RELEVANT_MESSAGE


# -- generation grounding guardrail -------------------------------------------


def test_check_grounding_passes_when_grounded(monkeypatch):
    _patch_config(monkeypatch)
    context = "RAG grounds answers in retrieved context. The return window is 30 days."
    answer = "RAG grounds answers in the retrieved context."
    assert check_grounding(answer, context) is None


def test_check_grounding_rejects_ungrounded_answer(monkeypatch):
    _patch_config(monkeypatch)
    context = "RAG grounds answers in retrieved context."
    answer = "The moon is made of cheese and RAG loves pancakes."
    assert check_grounding(answer, context) == UNGROUNDED_MESSAGE


def test_check_grounding_uses_explicit_threshold(monkeypatch):
    _patch_config(monkeypatch)
    context = "RAG grounds answers in retrieved context."
    answer = "RAG grounds answers in the provided context about RAG retrieval."
    assert check_grounding(answer, context, min_groundedness=0.05) is None
    assert check_grounding(answer, context, min_groundedness=0.95) == UNGROUNDED_MESSAGE