from dataclasses import dataclass

from errors import ErrorCategory


@dataclass(frozen=True)
class Failure:
    error_type: str
    category: ErrorCategory
    message: str
    retryable: bool = False

    def __post_init__(self) -> None:
        if not self.error_type.strip():
            raise ValueError("error_type cannot be empty.")

        if not self.message.strip():
            raise ValueError("message cannot be empty.")
