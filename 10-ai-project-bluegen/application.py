from config import (
    load_llm_config,
    load_pricing_config,
    load_settings,
)
from container import DependencyContainer
from context import RequestContext
from cost import ModelPricing
from interfaces import (
    ProjectGeneratorInterface,
    TelemetryRecorderInterface,
)
from schemas import (
    GenerateBlueprintRequest,
    GenerateBlueprintResponse,
    GenerationTelemetryResponse,
)
from service import ProjectBlueprintService
from tracing import TraceContext


class Application:

    def __init__(
        self,
        blueprint_service: ProjectBlueprintService,
        telemetry_recorder: TelemetryRecorderInterface,
    ) -> None:

        self.blueprint_service = blueprint_service
        self.telemetry_recorder = telemetry_recorder

    def generate_blueprint(
        self,
        request: GenerateBlueprintRequest,
    ) -> GenerateBlueprintResponse:

        context = RequestContext.create()

        trace = TraceContext(
            context=context,
            recorder=self.telemetry_recorder,
        )

        result = (
            self.blueprint_service
            .generate_blueprint(
                request.project_idea,
                trace,
            )
        )

        telemetry = GenerationTelemetryResponse(
            request_id=result.telemetry.request_id,
            model=result.telemetry.model,
            prompt_version=result.telemetry.prompt_version,
            latency_seconds=result.telemetry.latency_seconds,
            input_tokens=result.telemetry.input_tokens,
            output_tokens=result.telemetry.output_tokens,
            total_tokens=result.telemetry.total_tokens,
        )

        return GenerateBlueprintResponse(
            blueprint=result.blueprint,
            telemetry=telemetry,
        )


def create_application(
    generator: ProjectGeneratorInterface | None = None,
    container: DependencyContainer | None = None,
) -> Application:

    if container is None:
        settings = load_settings()
        llm_config = load_llm_config()

        container = DependencyContainer(
            settings=settings,
            llm_config=llm_config,
        )

    telemetry_recorder = (
        container.create_telemetry_recorder()
    )

    if generator is None:

        model = container.llm_config.model

        pricing_config = load_pricing_config()

        if model not in pricing_config.models:
            raise ValueError(
                f"No pricing configuration found for "
                f"model: {model}"
            )

        model_pricing_config = pricing_config.models[model]

        model_pricing = ModelPricing(
            input_cost_per_million_tokens=(
                model_pricing_config
                .input_cost_per_million_tokens
            ),
            output_cost_per_million_tokens=(
                model_pricing_config
                .output_cost_per_million_tokens
            ),
        )

        prompt_manager = (
            container.create_prompt_manager()
        )

        llm = (
            container.create_structured_llm()
        )

        cost_calculator = (
            container.create_cost_calculator()
        )

        generator = container.create_generator(
            llm=llm,
            prompt_manager=prompt_manager,
            telemetry_recorder=telemetry_recorder,
            cost_calculator=cost_calculator,
            model_pricing=model_pricing,
        )

    service = container.create_service(
        generator
    )

    return Application(
        blueprint_service=service,
        telemetry_recorder=telemetry_recorder,
    )