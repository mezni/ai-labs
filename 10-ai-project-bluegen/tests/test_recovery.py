import pytest

from errors import ErrorCategories, ErrorCategory
from failure import Failure
from recovery import (
    RecoveryAction,
    RecoveryDecision,
    RecoveryStrategy,
)


def test_timeout_failure_uses_fallback():
    strategy = RecoveryStrategy()

    failure = Failure(
        error_type="TimeoutError",
        category=ErrorCategories.TIMEOUT,
        message="LLM request timed out.",
        retryable=True,
    )

    decision = strategy.recover(failure)

    assert decision.action == RecoveryAction.FALLBACK


def test_llm_failure_escalates():
    strategy = RecoveryStrategy()

    failure = Failure(
        error_type="ProviderError",
        category=ErrorCategories.LLM,
        message="LLM provider failed.",
        retryable=True,
    )

    decision = strategy.recover(failure)

    assert decision.action == RecoveryAction.ESCALATE


def test_validation_failure_fails():
    strategy = RecoveryStrategy()

    failure = Failure(
        error_type="ValidationError",
        category=ErrorCategories.VALIDATION,
        message="Invalid project idea.",
        retryable=False,
    )

    decision = strategy.recover(failure)

    assert decision.action == RecoveryAction.FAIL


def test_strategy_matches_category_by_value_not_identity():
    strategy = RecoveryStrategy()

    equivalent_category = ErrorCategory(value="timeout")

    assert equivalent_category is not ErrorCategories.TIMEOUT

    failure = Failure(
        error_type="TimeoutError",
        category=equivalent_category,
        message="Same value, different instance.",
        retryable=True,
    )

    decision = strategy.recover(failure)

    assert decision.action == RecoveryAction.FALLBACK


def test_unknown_category_fails_closed():
    strategy = RecoveryStrategy()

    failure = Failure(
        error_type="SomethingNew",
        category=ErrorCategory(value="not-a-known-category"),
        message="Unrecognised failure kind.",
        retryable=True,
    )

    decision = strategy.recover(failure)

    assert decision.action == RecoveryAction.FAIL


def test_recovery_decision_requires_reason():
    with pytest.raises(ValueError):
        RecoveryDecision(
            action=RecoveryAction.FAIL,
            reason="",
        )


def test_recovery_decision_is_immutable():
    decision = RecoveryDecision(
        action=RecoveryAction.FAIL,
        reason="Failure cannot be recovered.",
    )

    with pytest.raises(AttributeError):
        decision.action = RecoveryAction.FALLBACK
