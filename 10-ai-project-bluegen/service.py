from collections.abc import Callable
from typing import TYPE_CHECKING

from interfaces import ProjectGeneratorInterface
from retry_executor import RetryExecutor
from telemetry import GenerationResult

if TYPE_CHECKING:
    from tracing import TraceContext


class ProjectBlueprintService:

    def __init__(
        self,
        generator: ProjectGeneratorInterface,
        retry_executor: RetryExecutor | None = None,
    ) -> None:

        self.generator = generator
        self.retry_executor = (
            retry_executor or RetryExecutor()
        )

    def generate_blueprint(
        self,
        project_idea: str,
        trace: "TraceContext",
        fallback: Callable[[], GenerationResult] | None = None,
    ) -> GenerationResult:

        outcome = self.retry_executor.execute(
            operation=lambda: self.generator.generate(
                project_idea,
                trace,
            ),
            fallback=fallback,
            request_id=trace.context.request_id,
        )

        return outcome.result
