from telemetry import TelemetryEvent
from telemetry_recorder import InMemoryTelemetryRecorder


def test_in_memory_recorder_stores_event():
    recorder = InMemoryTelemetryRecorder()

    event = TelemetryEvent(
        request_id="request-123",
        trace_id="trace-123",
        span_id="span-123",
        parent_span_id=None,
        event_type="generation",
        operation="project_blueprint_generation",
        status="success",
        latency_seconds=0.5,
        model="fake-model",
        prompt_version="v1",
    )

    recorder.record(event)

    assert len(recorder.events) == 1
    assert recorder.events[0] == event