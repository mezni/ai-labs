from dataclasses import dataclass
from typing import Any

from schemas import ProjectBlueprint


@dataclass(frozen=True)
class GenerationTelemetry:
    request_id: str
    model: str
    prompt_version: str
    latency_seconds: float
    input_tokens: int | None = None
    output_tokens: int | None = None
    total_tokens: int | None = None


@dataclass(frozen=True)
class TelemetryEvent:
    request_id: str
    trace_id: str
    span_id: str
    parent_span_id: str | None

    event_type: str
    operation: str
    status: str
    latency_seconds: float

    model: str | None = None
    prompt_version: str | None = None

    input_tokens: int | None = None
    output_tokens: int | None = None
    total_tokens: int | None = None

    input_cost: float | None = None
    output_cost: float | None = None
    total_cost: float | None = None

    error_type: str | None = None
    error_category: str | None = None

    attributes: dict[str, Any] | None = None


@dataclass(frozen=True)
class GenerationResult:
    blueprint: ProjectBlueprint
    telemetry: GenerationTelemetry