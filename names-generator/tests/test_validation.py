import pytest

from agents.name_generator_agent import validate_names


def test_validate_names_passes():
    names = ["Alpha", "Beta", "Gamma"]
    assert validate_names(names, 3) == names


def test_validate_names_wrong_count():
    with pytest.raises(ValueError, match="Expected exactly 10 names, got 3"):
        validate_names(["Alpha", "Beta", "Gamma"], 10)


def test_validate_names_empty():
    with pytest.raises(ValueError, match="No names generated"):
        validate_names([], 10)