import pytest

from metrics import InferenceMetrics


def test_ttft():
    metrics = InferenceMetrics(0, 1.2, 6.2, 100)

    assert metrics.ttft == pytest.approx(1.2)


def test_total_latency():
    metrics = InferenceMetrics(0, 1.2, 6.2, 100)

    assert metrics.total_latency == pytest.approx(6.2)


def test_tokens_per_second():
    metrics = InferenceMetrics(0, 1.2, 6.2, 100)

    assert metrics.tokens_per_second == pytest.approx(20.0)


def test_invalid_timestamp_order():
    metrics = InferenceMetrics(2, 1, 6, 100)

    with pytest.raises(ValueError):
        metrics.validate()


def test_invalid_token_count():
    metrics = InferenceMetrics(0, 1, 2, 0)

    with pytest.raises(ValueError):
        metrics.validate()
