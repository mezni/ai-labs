import pytest

from errors import ErrorCategories
from failure import Failure
from retry import RetryDecision, RetryPolicy


def create_retryable_failure() -> Failure:
    return Failure(
        error_type="TimeoutError",
        category=ErrorCategories.TIMEOUT,
        message="LLM request timed out.",
        retryable=True,
    )


def create_non_retryable_failure() -> Failure:
    return Failure(
        error_type="ValueError",
        category=ErrorCategories.VALIDATION,
        message="Invalid project idea.",
        retryable=False,
    )


def test_max_attempts_must_be_at_least_one():
    with pytest.raises(ValueError):
        RetryPolicy(max_attempts=0)


def test_non_retryable_failure_returns_no_retry():
    policy = RetryPolicy(max_attempts=3)

    decision = policy.decide(
        create_non_retryable_failure(),
        attempt=1,
    )

    assert decision.retry is False
    assert decision.next_attempt is None
    assert decision.reason == "Failure is not retryable."


def test_retryable_failure_returns_retry_decision():
    policy = RetryPolicy(max_attempts=3)

    decision = policy.decide(
        create_retryable_failure(),
        attempt=1,
    )

    assert decision.retry is True
    assert decision.next_attempt == 2


def test_second_attempt_can_retry():
    policy = RetryPolicy(max_attempts=3)

    decision = policy.decide(
        create_retryable_failure(),
        attempt=2,
    )

    assert decision.retry is True
    assert decision.next_attempt == 3


def test_max_attempts_returns_no_retry():
    policy = RetryPolicy(max_attempts=3)

    decision = policy.decide(
        create_retryable_failure(),
        attempt=3,
    )

    assert decision.retry is False
    assert decision.next_attempt is None
    assert decision.reason == "Maximum retry attempts reached."


def test_attempt_must_be_at_least_one():
    policy = RetryPolicy()

    with pytest.raises(ValueError):
        policy.decide(
            create_retryable_failure(),
            attempt=0,
        )


def test_retry_decision_requires_reason():
    with pytest.raises(ValueError):
        RetryDecision(
            retry=True,
            next_attempt=2,
            reason="",
        )


def test_retry_decision_requires_next_attempt_when_retrying():
    with pytest.raises(ValueError):
        RetryDecision(
            retry=True,
            next_attempt=None,
            reason="Retry requested.",
        )


def test_retry_decision_rejects_next_attempt_when_not_retrying():
    with pytest.raises(ValueError):
        RetryDecision(
            retry=False,
            next_attempt=2,
            reason="Do not retry.",
        )


def test_retry_decision_is_immutable():
    decision = RetryDecision(
        retry=True,
        next_attempt=2,
        reason="Retry requested.",
    )

    with pytest.raises(AttributeError):
        decision.retry = False
