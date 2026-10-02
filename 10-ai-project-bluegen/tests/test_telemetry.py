from telemetry import TelemetryEvent


def test_telemetry_event_contains_observability_data():
    event = TelemetryEvent(
        request_id="request-123",
        trace_id="trace-123",
        span_id="span-123",
        parent_span_id=None,
        event_type="generation",
        operation="project_blueprint_generation",
        status="success",
        latency_seconds=0.42,
        model="fake-model",
        prompt_version="v1",
        input_tokens=100,
        output_tokens=50,
        total_tokens=150,
        total_cost=0.0002,
    )

    assert event.request_id == "request-123"
    assert event.trace_id == "trace-123"
    assert event.span_id == "span-123"
    assert event.event_type == "generation"
    assert event.operation == "project_blueprint_generation"
    assert event.status == "success"
    assert event.total_tokens == 150
    assert event.total_cost == 0.0002


def test_telemetry_event_can_represent_tool_call():
    event = TelemetryEvent(
        request_id="request-456",
        trace_id="trace-456",
        span_id="span-456",
        parent_span_id=None,
        event_type="tool_call",
        operation="customer_lookup",
        status="success",
        latency_seconds=0.15,
    )

    assert event.event_type == "tool_call"
    assert event.operation == "customer_lookup"
    assert event.model is None
    assert event.input_tokens is None