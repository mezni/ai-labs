import pytest

from analyzer import (
    TokenizerAnalyzer,
    ContextBudget,
)


def test_tokenizer_counts_tokens():
    analyzer = TokenizerAnalyzer()

    count = analyzer.count_tokens("Hello world")

    assert count > 0


def test_tokenizer_encode_decode():
    analyzer = TokenizerAnalyzer()

    text = "Learning LLM engineering"

    token_ids = analyzer.encode(text)
    decoded = analyzer.decode(token_ids)

    assert decoded == text


def test_context_budget_remaining():
    budget = ContextBudget(
        context_limit=32000,
        reserved_output_tokens=4000,
    )

    assert budget.calculate_remaining(10000) == 18000


def test_context_budget_exceeded():
    budget = ContextBudget(
        context_limit=32000,
        reserved_output_tokens=4000,
    )

    assert budget.can_fit(30000) is False


def test_negative_input_rejected():
    budget = ContextBudget(32000, 4000)

    with pytest.raises(ValueError):
        budget.calculate_remaining(-1)


def test_invalid_output_reservation():
    with pytest.raises(ValueError):
        ContextBudget(32000, 32000)
