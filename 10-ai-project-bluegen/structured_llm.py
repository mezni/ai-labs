from langchain_openai import ChatOpenAI

from config import LLMConfig, Settings
from interfaces import LLMUsage, StructuredLLMInterface
from schemas import ProjectBlueprint


class LangChainStructuredLLM(StructuredLLMInterface):
    def __init__(
        self,
        config: LLMConfig,
        settings: Settings,
    ) -> None:

        self._model_name = config.model

        llm = ChatOpenAI(
            model=config.model,
            temperature=config.temperature,
            max_tokens=config.max_tokens,
            base_url=config.base_url,
            api_key=settings.openrouter_api_key,
        )

        self.llm = llm.with_structured_output(
            ProjectBlueprint,
            include_raw=True,
        )

    @property
    def model_name(self) -> str:
        return self._model_name

    def generate(
        self,
        messages: list[tuple[str, str]],
    ) -> tuple[ProjectBlueprint, LLMUsage]:

        response = self.llm.invoke(messages)

        blueprint = response["parsed"]
        raw_response = response["raw"]

        usage_metadata = getattr(
            raw_response,
            "usage_metadata",
            None,
        )

        if usage_metadata is None:
            usage = LLMUsage()
        else:
            usage = LLMUsage(
                input_tokens=usage_metadata.get("input_tokens"),
                output_tokens=usage_metadata.get("output_tokens"),
                total_tokens=usage_metadata.get("total_tokens"),
            )

        return blueprint, usage