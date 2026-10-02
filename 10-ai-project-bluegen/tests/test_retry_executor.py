import pytest

from backoff import ExponentialBackoff
from errors import ErrorCategories
from exceptions import ProjectGenerationError
from failure import Failure
from recovery import RecoveryAction
from retry import RetryPolicy
from retry_executor import RetryExecutor


class RecordingSleeper:
    def __init__(self) -> None:
        self.delays: list[float] = []

    def __call__(self, delay: float) -> None:
        self.delays.append(delay)


def _failure(
    category=ErrorCategories.LLM,
    retryable: bool = True,
) -> Failure:
    return Failure(
        error_type="ProviderError",
        category=category,
        message="Provider failed.",
        retryable=retryable,
    )


def _failing_operation(
    failure: Failure,
    times: int = 1,
):
    calls = {"count": 0}

    def operation() -> str:
        calls["count"] += 1

        if calls["count"] <= times:
            raise ProjectGenerationError(
                "Generation failed.",
                failure=failure,
            )

        return "ok"

    return operation, calls


def test_succeeds_on_first_attempt_without_sleeping() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(sleep=sleeper)

    calls = {"count": 0}

    def operation() -> str:
        calls["count"] += 1
        return "ok"

    outcome = executor.execute(operation)

    assert outcome.result == "ok"
    assert outcome.attempts == 1
    assert outcome.recovery_action is None
    assert calls["count"] == 1
    assert sleeper.delays == []


def test_retries_retryable_failure_then_succeeds() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(
        retry_policy=RetryPolicy(max_attempts=3),
        sleep=sleeper,
    )

    operation, calls = _failing_operation(
        _failure(),
        times=2,
    )

    outcome = executor.execute(operation)

    assert outcome.result == "ok"
    assert outcome.attempts == 3
    assert calls["count"] == 3
    assert sleeper.delays == [1.0, 2.0]


def test_backoff_delays_follow_exponential_sequence() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(
        retry_policy=RetryPolicy(max_attempts=5),
        backoff=ExponentialBackoff(
            initial_delay_seconds=0.5,
            multiplier=3.0,
            max_delay_seconds=100.0,
        ),
        sleep=sleeper,
    )

    operation, _ = _failing_operation(
        _failure(),
        times=99,
    )

    with pytest.raises(ProjectGenerationError):
        executor.execute(operation)

    assert sleeper.delays == [0.5, 1.5, 4.5, 13.5]


def test_stops_at_max_attempts_and_reraises() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(
        retry_policy=RetryPolicy(max_attempts=3),
        sleep=sleeper,
    )

    operation, calls = _failing_operation(
        _failure(),
        times=99,
    )

    with pytest.raises(ProjectGenerationError):
        executor.execute(operation)

    assert calls["count"] == 3
    assert len(sleeper.delays) == 2


def test_does_not_retry_non_retryable_failure() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(sleep=sleeper)

    operation, calls = _failing_operation(
        _failure(
            category=ErrorCategories.VALIDATION,
            retryable=False,
        ),
        times=99,
    )

    with pytest.raises(ProjectGenerationError):
        executor.execute(operation)

    assert calls["count"] == 1
    assert sleeper.delays == []


def test_unclassified_error_propagates_without_retrying() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(sleep=sleeper)

    calls = {"count": 0}

    def operation() -> str:
        calls["count"] += 1
        raise ProjectGenerationError("No failure attached.")

    with pytest.raises(ProjectGenerationError):
        executor.execute(operation)

    assert calls["count"] == 1
    assert sleeper.delays == []


def test_timeout_triggers_fallback_operation() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(
        retry_policy=RetryPolicy(max_attempts=1),
        sleep=sleeper,
    )

    operation, _ = _failing_operation(
        _failure(category=ErrorCategories.TIMEOUT),
        times=99,
    )

    outcome = executor.execute(
        operation,
        fallback=lambda: "fallback-result",
    )

    assert outcome.result == "fallback-result"
    assert outcome.recovery_action == RecoveryAction.FALLBACK
    assert outcome.failure is not None
    assert (
        outcome.failure.category == ErrorCategories.TIMEOUT
    )


def test_llm_failure_escalates_without_fallback() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(
        retry_policy=RetryPolicy(max_attempts=1),
        sleep=sleeper,
    )

    operation, _ = _failing_operation(
        _failure(category=ErrorCategories.LLM),
        times=99,
    )

    fallback_calls = {"count": 0}

    def fallback() -> str:
        fallback_calls["count"] += 1
        return "fallback-result"

    with pytest.raises(ProjectGenerationError):
        executor.execute(operation, fallback=fallback)

    assert fallback_calls["count"] == 0


def test_fallback_action_without_fallback_reraises() -> None:
    sleeper = RecordingSleeper()

    executor = RetryExecutor(
        retry_policy=RetryPolicy(max_attempts=1),
        sleep=sleeper,
    )

    operation, _ = _failing_operation(
        _failure(category=ErrorCategories.TIMEOUT),
        times=99,
    )

    with pytest.raises(ProjectGenerationError):
        executor.execute(operation)


def test_retry_outcome_is_immutable() -> None:
    from retry_executor import RetryOutcome

    outcome = RetryOutcome(
        result="ok",
        attempts=1,
        recovery_action=None,
        failure=None,
    )

    with pytest.raises(AttributeError):
        outcome.attempts = 5
