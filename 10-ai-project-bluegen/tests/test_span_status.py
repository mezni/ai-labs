import pytest

from span_status import SpanStatus, SpanStatuses


def test_span_status_requires_value():
    with pytest.raises(ValueError):
        SpanStatus("")


def test_success_status():
    assert SpanStatuses.SUCCESS.value == "success"


def test_error_status():
    assert SpanStatuses.ERROR.value == "error"


def test_span_status_is_immutable():
    status = SpanStatuses.SUCCESS

    with pytest.raises(AttributeError):
        status.value = "changed"
