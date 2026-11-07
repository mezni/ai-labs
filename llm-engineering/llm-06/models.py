from dataclasses import dataclass


@dataclass(frozen=True)
class LLMResult:
    content: str
    model: str
    input_tokens: int
    output_tokens: int
    finish_reason: str | None

    @property
    def total_tokens(self) -> int:
        return self.input_tokens + self.output_tokens
