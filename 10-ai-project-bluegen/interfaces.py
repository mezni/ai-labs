from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import TYPE_CHECKING

from schemas import ProjectBlueprint
from telemetry import GenerationResult, TelemetryEvent

if TYPE_CHECKING:
    from tracing import TraceContext


@dataclass(frozen=True)
class LLMUsage:
    input_tokens: int | None = None
    output_tokens: int | None = None
    total_tokens: int | None = None


class ProjectGeneratorInterface(ABC):

    @abstractmethod
    def generate(
        self,
        project_idea: str,
        trace: "TraceContext",
    ) -> GenerationResult:
        """Generate a project blueprint."""
        raise NotImplementedError


class StructuredLLMInterface(ABC):

    @property
    @abstractmethod
    def model_name(self) -> str:
        """Return the configured model name."""
        raise NotImplementedError

    @abstractmethod
    def generate(
        self,
        messages: list[tuple[str, str]],
    ) -> tuple[ProjectBlueprint, LLMUsage]:
        """Generate a structured project blueprint and usage."""
        raise NotImplementedError


class PromptManagerInterface(ABC):

    @abstractmethod
    def get_system_prompt(self) -> str:
        """Return the system prompt."""
        raise NotImplementedError

    @abstractmethod
    def build_user_prompt(
        self,
        project_idea: str,
    ) -> str:
        """Build the user prompt."""
        raise NotImplementedError

    @abstractmethod
    def get_version(self) -> str:
        """Return the active prompt version."""
        raise NotImplementedError


class TelemetryRecorderInterface(ABC):

    @abstractmethod
    def record(
        self,
        event: TelemetryEvent,
    ) -> None:
        """Record an observability event."""
        raise NotImplementedError