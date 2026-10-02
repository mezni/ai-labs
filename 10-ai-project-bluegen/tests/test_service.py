import pytest

from context import RequestContext
from cost import CostCalculator, ModelPricing
from exceptions import ProjectGenerationError
from generator import ProjectGenerator
from retry import RetryPolicy
from retry_executor import RetryExecutor
from service import ProjectBlueprintService
from telemetry_recorder import InMemoryTelemetryRecorder
from tests.fakes import (
    FakeProjectGenerator,
    FakePromptManager,
    FlakyLLM,
)
from tracing import TraceContext


def _build_real_generator(llm):
    return ProjectGenerator(
        llm=llm,
        prompt_manager=FakePromptManager(),
        telemetry_recorder=InMemoryTelemetryRecorder(),
        cost_calculator=CostCalculator(),
        model_pricing=ModelPricing(
            input_cost_per_million_tokens=0.0,
            output_cost_per_million_tokens=0.0,
        ),
    )


def test_service_generates_blueprint() -> None:
    generator = FakeProjectGenerator()
    service = ProjectBlueprintService(generator)

    context = RequestContext.create()

    trace = TraceContext(
        context=context,
        recorder=InMemoryTelemetryRecorder(),
    )

    result = service.generate_blueprint(
        "Build an AI document classifier.",
        trace,
    )

    assert result.blueprint.project_name == "Test Project"
    assert result.blueprint.business_outcome == "Test business outcome."
    assert result.telemetry.request_id == context.request_id


def test_service_retries_flaky_generator_and_succeeds() -> None:
    llm = FlakyLLM(failures_before_success=2)

    recorder = InMemoryTelemetryRecorder()

    service = ProjectBlueprintService(
        _build_real_generator(llm),
        retry_executor=RetryExecutor(
            retry_policy=RetryPolicy(max_attempts=3),
            sleep=lambda delay: None,
        ),
    )

    context = RequestContext.create()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    result = service.generate_blueprint(
        "Build an AI document classifier.",
        trace,
    )

    assert result.blueprint.project_name == "Recovered Project"
    assert llm.attempts == 3

    events = recorder.events

    assert len(events) == 3

    assert [event.status for event in events] == [
        "error",
        "error",
        "success",
    ]

    assert len({event.trace_id for event in events}) == 1

    assert all(
        event.request_id == context.request_id
        for event in events
    )

    assert len({event.span_id for event in events}) == 3

    assert all(
        event.error_category == "llm"
        for event in events[:2]
    )


def test_service_reraises_when_retries_exhausted() -> None:
    llm = FlakyLLM(failures_before_success=99)

    recorder = InMemoryTelemetryRecorder()

    service = ProjectBlueprintService(
        _build_real_generator(llm),
        retry_executor=RetryExecutor(
            retry_policy=RetryPolicy(max_attempts=2),
            sleep=lambda delay: None,
        ),
    )

    trace = TraceContext(
        context=RequestContext.create(),
        recorder=recorder,
    )

    with pytest.raises(ProjectGenerationError):
        service.generate_blueprint(
            "Build an AI document classifier.",
            trace,
        )

    assert llm.attempts == 2
    assert len(recorder.events) == 2
    assert trace.current_span_id is None
