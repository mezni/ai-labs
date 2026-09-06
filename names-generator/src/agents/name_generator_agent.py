from pydantic import BaseModel

from llm.llm_client import LLMClient, LLMResponse
from llm.prompt_manager import PromptManager
from utils.config_loader import load_agent_config
from utils.guardrails import (
    InputGuardrailViolation,
    OutputGuardrailViolation,
    validate_input_idea,
    validate_output_names,
)
from utils.json_utils import extract_json
from utils.logger import get_logger
from utils.tracer import tracer

logger = get_logger("name_generator_agent")


class ProjectName(BaseModel):
    name: str
    description: str
    reason: str


class NameCandidates(BaseModel):
    candidates: list[ProjectName]


def validate_names(names: list[ProjectName], expected: int) -> list[ProjectName]:
    if not names:
        raise ValueError("No names generated")

    if len(names) != expected:
        raise ValueError(f"Expected exactly {expected} names, got {len(names)}")

    return names


class NameGeneratorAgent:
    def __init__(self, llm: LLMClient | None = None, prompts: PromptManager | None = None):
        self.llm = llm or LLMClient()
        self.prompts = prompts or PromptManager()
        agent_config = load_agent_config()
        self.expected = int(agent_config["name_generator"].get("max_items", 10))
        self.max_attempts = int(agent_config["name_generator"].get("max_attempts", 3))
        self.max_idea_length = int(
            self.llm.config.get("name_generation", {}).get("max_idea_length", 1000)
        )

    def _estimate_cost(self, response: LLMResponse) -> float:
        pricing = self.llm.config.get("pricing", {})
        in_per_million = float(pricing.get("input_per_million", 0.0))
        out_per_million = float(pricing.get("output_per_million", 0.0))
        cost = (response.input_tokens or 0) / 1e6 * in_per_million
        cost += (response.output_tokens or 0) / 1e6 * out_per_million
        return round(cost, 6)

    def _generate_once(self, idea: str) -> list[ProjectName]:
        with tracer.span("build_prompt"):
            prompt = self.prompts.build(
                "name_generation", idea=idea, count=self.expected
            )

        logger.info("Requesting name candidates for idea: %s", idea[:80])

        with tracer.span("llm_call", model=self.llm.config["model"]) as llm_span:
            response = self.llm.complete([{"role": "user", "content": prompt}])
            llm_span.add(
                input_tokens=response.input_tokens,
                output_tokens=response.output_tokens,
                cost=self._estimate_cost(response),
            )

        with tracer.span("parse_response"):
            payload = extract_json(response.content)
            candidates = NameCandidates.model_validate(payload)

        with tracer.span("validate_output"):
            return validate_output_names(candidates.candidates, self.expected)

    def generate(self, project_idea: str, max_length: int | None = None) -> list[ProjectName]:
        with tracer.span("generate_names", idea_hint=project_idea[:60]):
            with tracer.span("validate_input"):
                idea = validate_input_idea(
                    project_idea, max_length or self.max_idea_length
                )

            for attempt in range(1, self.max_attempts + 1):
                try:
                    return self._generate_once(idea)
                except (OutputGuardrailViolation, ValueError, RuntimeError) as error:
                    logger.warning(
                        "Attempt %s/%s failed: %s", attempt, self.max_attempts, error
                    )

            raise RuntimeError("Unable to generate names")