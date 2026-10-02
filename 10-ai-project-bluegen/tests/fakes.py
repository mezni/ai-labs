from interfaces import (
    LLMUsage,
    ProjectGeneratorInterface,
    PromptManagerInterface,
)
from schemas import ProjectBlueprint
from telemetry import GenerationResult, GenerationTelemetry
from tracing import TraceContext


class FakeProjectGenerator(ProjectGeneratorInterface):

    def generate(
        self,
        project_idea: str,
        trace: TraceContext,
    ) -> GenerationResult:

        return GenerationResult(
            blueprint=ProjectBlueprint(
                project_name="Test Project",
                business_outcome=(
                    "Test business outcome."
                ),
            ),
            telemetry=GenerationTelemetry(
                request_id=trace.context.request_id,
                model="fake-model",
                prompt_version="test-v1",
                latency_seconds=0.01,
            ),
        )


class FakeLLM:

    @property
    def model_name(self) -> str:
        return "fake-model"

    def generate(self, messages):
        return (
            ProjectBlueprint(
                project_name="Test Project",
                business_outcome="Test business outcome.",
            ),
            LLMUsage(
                input_tokens=100,
                output_tokens=50,
                total_tokens=150,
            ),
        )


class FlakyLLM:
    """Fails a set number of times, then succeeds."""

    def __init__(
        self,
        failures_before_success: int,
        error: Exception | None = None,
    ) -> None:

        self.failures_before_success = (
            failures_before_success
        )
        self.error = error or RuntimeError("LLM failed.")
        self.attempts = 0

    @property
    def model_name(self) -> str:
        return "flaky-model"

    def generate(self, messages):
        self.attempts += 1

        if self.attempts <= self.failures_before_success:
            raise self.error

        return (
            ProjectBlueprint(
                project_name="Recovered Project",
                business_outcome=(
                    "Recovered business outcome."
                ),
            ),
            LLMUsage(
                input_tokens=10,
                output_tokens=5,
                total_tokens=15,
            ),
        )


class FakePromptManager(PromptManagerInterface):

    def get_system_prompt(self) -> str:
        return "Fake system prompt."

    def build_user_prompt(
        self,
        project_idea: str,
    ) -> str:
        return f"Fake prompt: {project_idea}"

    def get_version(self) -> str:
        return "test-v1"