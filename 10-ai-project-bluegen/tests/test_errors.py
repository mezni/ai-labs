import pytest

from errors import ErrorCategory, ErrorCategories


def test_error_category_requires_value():
    with pytest.raises(ValueError):
        ErrorCategory("")


def test_error_categories():
    assert ErrorCategories.VALIDATION.value == "validation"
    assert ErrorCategories.CONFIGURATION.value == "configuration"
    assert ErrorCategories.LLM.value == "llm"
    assert ErrorCategories.TIMEOUT.value == "timeout"
    assert ErrorCategories.INTERNAL.value == "internal"


def test_error_category_is_immutable():
    category = ErrorCategories.LLM

    with pytest.raises(AttributeError):
        category.value = "changed"
