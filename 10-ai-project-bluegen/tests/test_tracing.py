import pytest

from context import RequestContext
from errors import ErrorCategories
from failure import Failure
from operations import Operation, Operations
from span_status import SpanStatuses
from telemetry_recorder import InMemoryTelemetryRecorder
from tracing import Span, TraceContext


def test_span_records_success_event():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    span = Span.start(
        context=context,
        recorder=recorder,
        operation=Operation(
            event_type="llm",
            name="test_operation",
        ),
    )

    span.finish(
        status=SpanStatuses.SUCCESS,
        model="fake-model",
    )

    assert len(recorder.events) == 1

    event = recorder.events[0]

    assert event.request_id == context.request_id
    assert event.trace_id == context.trace_id
    assert event.span_id
    assert event.event_type == "llm"
    assert event.operation == "test_operation"
    assert event.status == "success"
    assert event.model == "fake-model"
    assert event.latency_seconds >= 0


def test_span_records_parent_span():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    parent = Span.start(
        context=context,
        recorder=recorder,
        operation=Operation(
            event_type="agent",
            name="agent_execution",
        ),
    )

    child = Span.start(
        context=context,
        recorder=recorder,
        operation=Operation(
            event_type="llm",
            name="llm_generation",
        ),
        parent_span_id=parent.span_id,
    )

    child.finish(status=SpanStatuses.SUCCESS)
    parent.finish(status=SpanStatuses.SUCCESS)

    assert len(recorder.events) == 2

    child_event = recorder.events[0]
    parent_event = recorder.events[1]

    assert parent_event.parent_span_id is None
    assert child_event.parent_span_id == parent_event.span_id
    assert child_event.trace_id == parent_event.trace_id


def test_span_records_error_event():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    span = Span.start(
        context=context,
        recorder=recorder,
        operation=Operation(
            event_type="llm",
            name="test_operation",
        ),
    )

    span.finish(
        status=SpanStatuses.ERROR,
        failure=Failure(
            error_type="TimeoutError",
            category=ErrorCategories.TIMEOUT,
            message="LLM request timed out.",
        ),
    )

    event = recorder.events[0]

    assert event.status == "error"
    assert event.error_type == "TimeoutError"
    assert event.latency_seconds >= 0


def test_trace_context_creates_parent_child_relationships():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    parent = trace.start_span(
        Operation(
            event_type="agent",
            name="agent_execution",
        )
    )

    child = trace.start_span(
        Operation(
            event_type="llm",
            name="llm_generation",
        )
    )

    assert parent.parent_span_id is None
    assert child.parent_span_id == parent.span_id

    assert trace.current_span_id == child.span_id


def test_trace_context_finishes_spans_in_lifo_order():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    parent = trace.start_span(
        Operation(
            event_type="agent",
            name="agent_execution",
        )
    )

    child = trace.start_span(
        Operation(
            event_type="llm",
            name="llm_generation",
        )
    )

    with pytest.raises(RuntimeError, match="LIFO"):
        trace.finish_span(
            parent,
            status=SpanStatuses.SUCCESS,
        )

    trace.finish_span(
        child,
        status=SpanStatuses.SUCCESS,
    )

    trace.finish_span(
        parent,
        status=SpanStatuses.SUCCESS,
    )

    assert trace.current_span_id is None


def test_trace_context_records_nested_events():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    parent = trace.start_span(
        Operation(
            event_type="agent",
            name="agent_execution",
        )
    )

    child = trace.start_span(
        Operation(
            event_type="llm",
            name="llm_generation",
        )
    )

    trace.finish_span(
        child,
        status=SpanStatuses.SUCCESS,
    )

    trace.finish_span(
        parent,
        status=SpanStatuses.SUCCESS,
    )

    assert len(recorder.events) == 2

    # Events are recorded in LIFO order (child first, then parent)
    child_event = recorder.events[0]
    parent_event = recorder.events[1]

    assert parent_event.trace_id == child_event.trace_id
    assert parent_event.span_id == child_event.parent_span_id


def test_span_records_attributes():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    span = trace.start_span(
        Operations.LLM_PROJECT_BLUEPRINT_GENERATION,
    )

    trace.finish_span(
        span,
        status=SpanStatuses.SUCCESS,
        attributes={
            "provider": "openrouter",
            "temperature": 0.2,
        },
    )

    event = recorder.events[0]

    assert event.attributes == {
        "provider": "openrouter",
        "temperature": 0.2,
    }


def test_span_attributes_are_optional():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    span = trace.start_span(
        Operations.LLM_PROJECT_BLUEPRINT_GENERATION,
    )

    trace.finish_span(
        span,
        status=SpanStatuses.SUCCESS,
    )

    event = recorder.events[0]

    assert event.attributes is None


def test_span_records_error_category():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    span = trace.start_span(
        Operations.LLM_PROJECT_BLUEPRINT_GENERATION,
    )

    trace.finish_span(
        span,
        status=SpanStatuses.ERROR,
        failure=Failure(
            error_type="ProviderError",
            category=ErrorCategories.LLM,
            message="LLM generation failed.",
        ),
    )

    event = recorder.events[0]

    assert event.status == "error"
    assert event.error_type == "ProviderError"
    assert event.error_category == "llm"


def test_span_records_failure():
    context = RequestContext.create()
    recorder = InMemoryTelemetryRecorder()

    trace = TraceContext(
        context=context,
        recorder=recorder,
    )

    span = trace.start_span(
        Operations.LLM_PROJECT_BLUEPRINT_GENERATION,
    )

    failure = Failure(
        error_type="ProviderError",
        category=ErrorCategories.LLM,
        message="LLM generation failed.",
        retryable=True,
    )

    trace.finish_span(
        span,
        status=SpanStatuses.ERROR,
        failure=failure,
    )

    event = recorder.events[0]

    assert event.status == "error"
    assert event.error_type == "ProviderError"
    assert event.error_category == "llm"
