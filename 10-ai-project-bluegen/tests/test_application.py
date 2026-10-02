import pytest

import application
from application import Application, create_application
from config import (
    LLMConfig,
    ModelPricingConfig,
    PricingConfig,
    Settings,
)
from container import DependencyContainer
from generator import ProjectGenerator
from interfaces import ProjectGeneratorInterface
from schemas import (
    GenerateBlueprintRequest,
    ProjectBlueprint,
)
from service import ProjectBlueprintService
from telemetry import (
    GenerationResult,
    GenerationTelemetry,
)
from telemetry_recorder import InMemoryTelemetryRecorder
from tests.fakes import FakeProjectGenerator
from tracing import TraceContext


class CapturingGenerator(ProjectGeneratorInterface):

    def __init__(self) -> None:
        self.trace: TraceContext | None = None

    def generate(
        self,
        project_idea: str,
        trace: TraceContext,
    ) -> GenerationResult:

        self.trace = trace

        return GenerationResult(
            blueprint=ProjectBlueprint(
                project_name="Test Project",
                business_outcome="Test outcome.",
            ),
            telemetry=GenerationTelemetry(
                request_id=trace.context.request_id,
                model="fake-model",
                prompt_version="test-v1",
                latency_seconds=0.01,
            ),
        )


def test_create_application_with_custom_generator() -> None:
    container = DependencyContainer(
        settings=Settings(
            openrouter_api_key="test-key",
        ),
        llm_config=LLMConfig(
            provider="openrouter",
            base_url="https://openrouter.ai/api/v1",
            model="openai/gpt-4o-mini",
            temperature=0.2,
            max_tokens=1000,
        ),
    )

    application = create_application(
        generator=FakeProjectGenerator(),
        container=container,
    )

    request = GenerateBlueprintRequest(
        project_idea="Build an AI document classifier."
    )

    result = application.generate_blueprint(request)

    assert result.blueprint.project_name == "Test Project"
    assert result.telemetry.request_id
    assert len(result.telemetry.request_id) == 36
    assert result.telemetry.model == "fake-model"
    assert result.telemetry.prompt_version == "test-v1"


def _build_container(model: str) -> DependencyContainer:
    return DependencyContainer(
        settings=Settings(
            openrouter_api_key="test-key",
        ),
        llm_config=LLMConfig(
            provider="openrouter",
            base_url="https://openrouter.ai/api/v1",
            model=model,
            temperature=0.2,
            max_tokens=1000,
        ),
    )


def test_create_application_builds_generator_from_container(
    monkeypatch,
) -> None:
    monkeypatch.setattr(
        application,
        "load_pricing_config",
        lambda: PricingConfig(
            models={
                "openai/gpt-4o-mini": ModelPricingConfig(
                    input_cost_per_million_tokens=1.5,
                    output_cost_per_million_tokens=6.0,
                )
            }
        ),
    )

    container = _build_container("openai/gpt-4o-mini")

    app = create_application(container=container)

    assert isinstance(
        app.blueprint_service.generator,
        ProjectGenerator,
    )
    assert (
        app.blueprint_service.generator.model_pricing
        .input_cost_per_million_tokens
        == 1.5
    )


def test_create_application_raises_when_pricing_missing(
    monkeypatch,
) -> None:
    monkeypatch.setattr(
        application,
        "load_pricing_config",
        lambda: PricingConfig(models={}),
    )

    container = _build_container("openai/gpt-4o-mini")

    with pytest.raises(ValueError, match="openai/gpt-4o-mini"):
        create_application(container=container)


def test_application_creates_trace_context() -> None:
    generator = CapturingGenerator()

    service = ProjectBlueprintService(generator)

    recorder = InMemoryTelemetryRecorder()

    application = Application(
        blueprint_service=service,
        telemetry_recorder=recorder,
    )

    request = GenerateBlueprintRequest(
        project_idea="Build an AI document classifier."
    )

    response = application.generate_blueprint(request)

    assert generator.trace is not None
    assert (
        generator.trace.context.request_id
        == response.telemetry.request_id
    )
    assert generator.trace.recorder is recorder
    assert generator.trace.current_span_id is None
