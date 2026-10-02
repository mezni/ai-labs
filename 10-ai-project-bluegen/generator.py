import logging

from cost import CostCalculator, ModelPricing
from errors import ErrorCategories
from exceptions import ProjectGenerationError
from failure import Failure
from interfaces import (
    ProjectGeneratorInterface,
    PromptManagerInterface,
    StructuredLLMInterface,
    TelemetryRecorderInterface,
)
from logger import StructuredLogger
from operations import Operations
from span_status import SpanStatuses
from telemetry import (
    GenerationResult,
    GenerationTelemetry,
)
from tracing import TraceContext


class ProjectGenerator(ProjectGeneratorInterface):

    def __init__(
        self,
        llm: StructuredLLMInterface,
        prompt_manager: PromptManagerInterface,
        telemetry_recorder: TelemetryRecorderInterface,
        cost_calculator: CostCalculator,
        model_pricing: ModelPricing,
    ) -> None:

        self.llm = llm
        self.prompt_manager = prompt_manager
        self.telemetry_recorder = telemetry_recorder
        self.cost_calculator = cost_calculator
        self.model_pricing = model_pricing

        self.logger = StructuredLogger(
            logging.getLogger(__name__)
        )

    def generate(
        self,
        project_idea: str,
        trace: TraceContext,
    ) -> GenerationResult:

        project_idea = project_idea.strip()

        if not project_idea:
            raise ValueError(
                "Project idea cannot be empty."
            )

        request_id = trace.context.request_id

        self.logger.info(
            "Starting blueprint generation",
            request_id=request_id,
            operation="generate_blueprint",
        )

        messages = [
            (
                "system",
                self.prompt_manager.get_system_prompt(),
            ),
            (
                "human",
                self.prompt_manager.build_user_prompt(
                    project_idea
                ),
            ),
        ]

        span = trace.start_span(
            Operations.LLM_PROJECT_BLUEPRINT_GENERATION
        )

        try:
            self.logger.info(
                "Calling structured LLM",
                request_id=request_id,
                operation="llm_generation",
            )

            blueprint, usage = self.llm.generate(messages)

            self.logger.info(
                "Blueprint generation completed",
                request_id=request_id,
                operation="generate_blueprint",
            )

        except Exception as exc:
            failure = Failure(
                error_type=type(exc).__name__,
                category=ErrorCategories.LLM,
                message="LLM generation failed.",
                retryable=True,
            )

            trace.finish_span(
                span,
                status=SpanStatuses.ERROR,
                model=self.llm.model_name,
                prompt_version=(
                    self.prompt_manager.get_version()
                ),
                failure=failure,
            )

            self.logger.exception(
                "Blueprint generation failed",
                request_id=request_id,
                operation="generate_blueprint",
            )

            raise ProjectGenerationError(
                "Failed to generate the project blueprint.",
                failure=failure,
            ) from exc

        latency_seconds = span.latency_seconds

        telemetry = GenerationTelemetry(
            request_id=request_id,
            model=self.llm.model_name,
            prompt_version=self.prompt_manager.get_version(),
            latency_seconds=latency_seconds,
            input_tokens=usage.input_tokens,
            output_tokens=usage.output_tokens,
            total_tokens=usage.total_tokens,
        )

        cost = None

        if (
            usage.input_tokens is not None
            and usage.output_tokens is not None
        ):
            cost = self.cost_calculator.calculate(
                pricing=self.model_pricing,
                input_tokens=usage.input_tokens,
                output_tokens=usage.output_tokens,
            )

        trace.finish_span(
            span,
            status=SpanStatuses.SUCCESS,
            model=self.llm.model_name,
            prompt_version=self.prompt_manager.get_version(),
            input_tokens=usage.input_tokens,
            output_tokens=usage.output_tokens,
            total_tokens=usage.total_tokens,
            input_cost=(
                cost.input_cost
                if cost is not None
                else None
            ),
            output_cost=(
                cost.output_cost
                if cost is not None
                else None
            ),
            total_cost=(
                cost.total_cost
                if cost is not None
                else None
            ),
            attributes={
                "provider": "openrouter",
            },
        )

        return GenerationResult(
            blueprint=blueprint,
            telemetry=telemetry,
        )