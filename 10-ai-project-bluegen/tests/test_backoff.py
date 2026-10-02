import pytest

from backoff import ExponentialBackoff


def test_first_attempt_uses_initial_delay():
    backoff = ExponentialBackoff(
        initial_delay_seconds=1.0,
        multiplier=2.0,
        max_delay_seconds=30.0,
    )

    assert backoff.calculate_delay(1) == 1.0


def test_delay_grows_exponentially():
    backoff = ExponentialBackoff(
        initial_delay_seconds=1.0,
        multiplier=2.0,
        max_delay_seconds=30.0,
    )

    assert backoff.calculate_delay(1) == 1.0
    assert backoff.calculate_delay(2) == 2.0
    assert backoff.calculate_delay(3) == 4.0
    assert backoff.calculate_delay(4) == 8.0


def test_delay_is_capped():
    backoff = ExponentialBackoff(
        initial_delay_seconds=1.0,
        multiplier=2.0,
        max_delay_seconds=5.0,
    )

    assert backoff.calculate_delay(1) == 1.0
    assert backoff.calculate_delay(2) == 2.0
    assert backoff.calculate_delay(3) == 4.0
    assert backoff.calculate_delay(4) == 5.0
    assert backoff.calculate_delay(5) == 5.0


def test_attempt_must_be_at_least_one():
    backoff = ExponentialBackoff()

    with pytest.raises(ValueError):
        backoff.calculate_delay(0)


def test_initial_delay_cannot_be_negative():
    with pytest.raises(ValueError):
        ExponentialBackoff(
            initial_delay_seconds=-1.0,
        )


def test_multiplier_must_be_at_least_one():
    with pytest.raises(ValueError):
        ExponentialBackoff(
            multiplier=0.5,
        )


def test_max_delay_cannot_be_negative():
    with pytest.raises(ValueError):
        ExponentialBackoff(
            max_delay_seconds=-1.0,
        )


def test_max_delay_cannot_be_less_than_initial_delay():
    with pytest.raises(ValueError):
        ExponentialBackoff(
            initial_delay_seconds=10.0,
            max_delay_seconds=5.0,
        )


def test_backoff_is_immutable():
    backoff = ExponentialBackoff()

    with pytest.raises(AttributeError):
        backoff.multiplier = 3.0
