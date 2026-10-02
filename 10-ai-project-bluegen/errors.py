from dataclasses import dataclass


@dataclass(frozen=True)
class ErrorCategory:
    value: str

    def __post_init__(self) -> None:
        if not self.value.strip():
            raise ValueError("Error category cannot be empty.")


class ErrorCategories:
    VALIDATION = ErrorCategory("validation")
    CONFIGURATION = ErrorCategory("configuration")
    LLM = ErrorCategory("llm")
    TIMEOUT = ErrorCategory("timeout")
    INTERNAL = ErrorCategory("internal")
