import pytest

from support_agent.tools import get_order_status


def test_known_order():
    result = get_order_status("4821")

    assert result["status"] == "shipped"
    assert result["estimated_delivery"] == "2026-10-12"


def test_unknown_order():
    result = get_order_status("9999")

    assert result["status"] == "not_found"


def test_rejects_invalid_order_id():
    with pytest.raises(ValueError):
        get_order_status("abc")
