import pytest
from pydantic import ValidationError

from schemas import (
    GenerateBlueprintRequest,
    GenerateBlueprintResponse,
    GenerationTelemetryResponse,
    ProjectBlueprint,
)


def test_generate_blueprint_request() -> None:
    request = GenerateBlueprintRequest(
        project_idea="Build an AI document classifier."
    )

    assert request.project_idea == (
        "Build an AI document classifier."
    )


def test_generate_blueprint_request_strips_whitespace() -> None:
    request = GenerateBlueprintRequest(
        project_idea="   Build an AI document classifier.   "
    )

    assert request.project_idea == (
        "Build an AI document classifier."
    )


def test_generate_blueprint_request_rejects_empty_input() -> None:
    with pytest.raises(ValidationError):
        GenerateBlueprintRequest(
            project_idea="   "
        )


def test_generate_blueprint_response() -> None:
    response = GenerateBlueprintResponse(
        blueprint=ProjectBlueprint(
            project_name="Document Classifier",
            business_outcome=(
                "Automatically classify corporate documents."
            ),
        ),
        telemetry=GenerationTelemetryResponse(
            request_id="test-request",
            model="openai/gpt-4o-mini",
            prompt_version="v1",
            latency_seconds=0.5,
        ),
    )

    assert response.blueprint.project_name == "Document Classifier"
    assert response.telemetry.prompt_version == "v1"