import pytest

from operations import Operation, Operations


def test_operation_requires_event_type():
    with pytest.raises(ValueError):
        Operation(
            event_type="",
            name="test_operation",
        )


def test_operation_requires_name():
    with pytest.raises(ValueError):
        Operation(
            event_type="llm",
            name="",
        )


def test_operation_is_immutable():
    operation = Operation(
        event_type="llm",
        name="test_operation",
    )

    with pytest.raises(AttributeError):
        operation.name = "changed"


def test_operation_catalogue_contains_llm_operation():
    operation = Operations.LLM_PROJECT_BLUEPRINT_GENERATION

    assert operation.event_type == "llm"
    assert operation.name == "project_blueprint_generation"


def test_operation_catalogue_contains_agent_operation():
    operation = Operations.AGENT_EXECUTION

    assert operation.event_type == "agent"
    assert operation.name == "agent_execution"
