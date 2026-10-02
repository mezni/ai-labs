from context import RequestContext
from cost import CostCalculator, ModelPricing
from generator import ProjectGenerator
from telemetry_recorder import InMemoryTelemetryRecorder
from tests.fakes import FakeLLM, FakePromptManager
from tracing import TraceContext


def test_generator_uses_injected_dependencies() -> None:
    recorder = InMemoryTelemetryRecorder()

    pricing = ModelPricing(
        input_cost_per_million_tokens=1.0,
        output_cost_per_million_tokens=2.0,
    )

    generator = ProjectGenerator(
        llm=FakeLLM(),
        prompt_manager=FakePromptManager(),
        telemetry_recorder=recorder,
        cost_calculator=CostCalculator(),
        model_pricing=pricing,
    )

    context = RequestContext.create()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    result = generator.generate(
        "Build an AI document classifier.",
        trace,
    )

    assert result.blueprint.project_name == "Test Project"

    assert (
        result.blueprint.business_outcome
        == "Test business outcome."
    )

    assert result.telemetry.model == "fake-model"

    assert (
        result.telemetry.prompt_version
        == "test-v1"
    )

    assert result.telemetry.input_tokens == 100
    assert result.telemetry.output_tokens == 50
    assert result.telemetry.total_tokens == 150

    assert len(recorder.events) == 1

    event = recorder.events[0]

    assert event.request_id == context.request_id
    assert event.operation == "project_blueprint_generation"
    assert event.status == "success"
    assert event.model == "fake-model"
    assert event.prompt_version == "test-v1"
    assert event.input_tokens == 100
    assert event.output_tokens == 50
    assert event.total_tokens == 150
    assert event.input_cost == 0.0001
    assert event.output_cost == 0.0001
    assert event.total_cost == 0.0002