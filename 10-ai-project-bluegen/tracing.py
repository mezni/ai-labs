from dataclasses import dataclass, field
import time
from typing import Any

from context import RequestContext
from failure import Failure
from operations import Operation
from span_status import SpanStatus
from telemetry import TelemetryEvent
from interfaces import TelemetryRecorderInterface


@dataclass
class Span:
    context: RequestContext
    recorder: TelemetryRecorderInterface
    operation: Operation
    span_id: str
    parent_span_id: str | None
    start_time: float

    @classmethod
    def start(
        cls,
        context: RequestContext,
        recorder: TelemetryRecorderInterface,
        operation: Operation,
        parent_span_id: str | None = None,
    ) -> "Span":
        return cls(
            context=context,
            recorder=recorder,
            operation=operation,
            span_id=context.create_span_id(),
            parent_span_id=parent_span_id,
            start_time=time.perf_counter(),
        )

    @property
    def latency_seconds(self) -> float:
        return time.perf_counter() - self.start_time

    def finish(
        self,
        *,
        status: SpanStatus,
        model: str | None = None,
        prompt_version: str | None = None,
        input_tokens: int | None = None,
        output_tokens: int | None = None,
        total_tokens: int | None = None,
        input_cost: float | None = None,
        output_cost: float | None = None,
        total_cost: float | None = None,
        failure: Failure | None = None,
        attributes: dict[str, Any] | None = None,
    ) -> None:
        latency = self.latency_seconds

        event = TelemetryEvent(
            request_id=self.context.request_id,
            trace_id=self.context.trace_id,
            span_id=self.span_id,
            parent_span_id=self.parent_span_id,
            event_type=self.operation.event_type,
            operation=self.operation.name,
            status=status.value,
            latency_seconds=latency,
            model=model,
            prompt_version=prompt_version,
            input_tokens=input_tokens,
            output_tokens=output_tokens,
            total_tokens=total_tokens,
            input_cost=input_cost,
            output_cost=output_cost,
            total_cost=total_cost,
            error_type=(
                failure.error_type
                if failure is not None
                else None
            ),
            error_category=(
                failure.category.value
                if failure is not None
                else None
            ),
            attributes=attributes,
        )

        self.recorder.record(event)


@dataclass
class TraceContext:
    context: RequestContext
    recorder: TelemetryRecorderInterface
    _span_stack: list[Span] = field(default_factory=list)

    def start_span(
        self,
        operation: Operation,
    ) -> Span:
        parent_span_id = (
            self._span_stack[-1].span_id
            if self._span_stack
            else None
        )

        span = Span.start(
            context=self.context,
            recorder=self.recorder,
            operation=operation,
            parent_span_id=parent_span_id,
        )

        self._span_stack.append(span)

        return span

    def finish_span(
        self,
        span: Span,
        *,
        status: SpanStatus,
        model: str | None = None,
        prompt_version: str | None = None,
        input_tokens: int | None = None,
        output_tokens: int | None = None,
        total_tokens: int | None = None,
        input_cost: float | None = None,
        output_cost: float | None = None,
        total_cost: float | None = None,
        failure: Failure | None = None,
        attributes: dict[str, Any] | None = None,
    ) -> None:
        if not self._span_stack:
            raise RuntimeError("No active spans.")

        if self._span_stack[-1] is not span:
            raise RuntimeError(
                "Span must be finished in LIFO order."
            )

        span.finish(
            status=status,
            model=model,
            prompt_version=prompt_version,
            input_tokens=input_tokens,
            output_tokens=output_tokens,
            total_tokens=total_tokens,
            input_cost=input_cost,
            output_cost=output_cost,
            total_cost=total_cost,
            failure=failure,
            attributes=attributes,
        )

        self._span_stack.pop()

    @property
    def current_span_id(self) -> str | None:
        if not self._span_stack:
            return None

        return self._span_stack[-1].span_id