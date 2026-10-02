from config import LLMConfig, Settings
from cost import CostCalculator, ModelPricing
from generator import ProjectGenerator
from interfaces import (
    ProjectGeneratorInterface,
    PromptManagerInterface,
    StructuredLLMInterface,
    TelemetryRecorderInterface,
)
from prompt_manager import PromptManager
from service import ProjectBlueprintService
from structured_llm import LangChainStructuredLLM
from telemetry_recorder import InMemoryTelemetryRecorder


class DependencyContainer:

    def __init__(
        self,
        settings: Settings,
        llm_config: LLMConfig,
    ) -> None:

        self.settings = settings
        self.llm_config = llm_config

    def create_prompt_manager(
        self,
    ) -> PromptManagerInterface:

        return PromptManager()

    def create_structured_llm(
        self,
    ) -> StructuredLLMInterface:

        return LangChainStructuredLLM(
            config=self.llm_config,
            settings=self.settings,
        )

    def create_telemetry_recorder(
        self,
    ) -> TelemetryRecorderInterface:

        return InMemoryTelemetryRecorder()

    def create_cost_calculator(self) -> CostCalculator:
        return CostCalculator()

    def create_generator(
        self,
        llm: StructuredLLMInterface,
        prompt_manager: PromptManagerInterface,
        telemetry_recorder: TelemetryRecorderInterface,
        cost_calculator: CostCalculator,
        model_pricing: ModelPricing,
    ) -> ProjectGeneratorInterface:

        return ProjectGenerator(
            llm=llm,
            prompt_manager=prompt_manager,
            telemetry_recorder=telemetry_recorder,
            cost_calculator=cost_calculator,
            model_pricing=model_pricing,
        )

    def create_service(
        self,
        generator: ProjectGeneratorInterface,
    ) -> ProjectBlueprintService:

        return ProjectBlueprintService(
            generator
        )