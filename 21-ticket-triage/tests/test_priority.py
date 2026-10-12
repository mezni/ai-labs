import pytest

from ticket_triage.priority import assess_priority


@pytest.mark.parametrize(
    ("ticket_text", "expected_priority"),
    [
        ("Our production service is down.", "critical"),
        ("I cannot renew because my payment failed.", "high"),
        ("I was charged twice for my subscription.", "medium"),
        ("How do I change my billing address?", "low"),
    ],
)
def test_assess_priority(ticket_text, expected_priority):
    result = assess_priority(ticket_text)

    assert result["priority"] == expected_priority
    assert isinstance(result["reason"], str)
    assert result["reason"]


def test_critical_priority_overrides_medium_keyword():
    result = assess_priority("Our production service is down and we need a refund.")

    assert result["priority"] == "critical"


@pytest.mark.parametrize("ticket_text", ["", "   ", "\n"])
def test_rejects_empty_ticket(ticket_text):
    with pytest.raises(ValueError):
        assess_priority(ticket_text)


def test_rejects_non_string_input():
    with pytest.raises(ValueError):
        assess_priority(None)
