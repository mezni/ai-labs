import pytest

from errors import ErrorCategories
from failure import Failure


def test_failure_requires_error_type():
    with pytest.raises(ValueError):
        Failure(
            error_type="",
            category=ErrorCategories.LLM,
            message="LLM request failed.",
        )


def test_failure_requires_message():
    with pytest.raises(ValueError):
        Failure(
            error_type="ProviderError",
            category=ErrorCategories.LLM,
            message="",
        )


def test_failure_defaults_to_not_retryable():
    failure = Failure(
        error_type="ValueError",
        category=ErrorCategories.VALIDATION,
        message="Invalid project idea.",
    )

    assert failure.retryable is False


def test_failure_can_be_retryable():
    failure = Failure(
        error_type="TimeoutError",
        category=ErrorCategories.TIMEOUT,
        message="LLM request timed out.",
        retryable=True,
    )

    assert failure.retryable is True


def test_failure_is_immutable():
    failure = Failure(
        error_type="ProviderError",
        category=ErrorCategories.LLM,
        message="Provider request failed.",
    )

    with pytest.raises(AttributeError):
        failure.retryable = True
