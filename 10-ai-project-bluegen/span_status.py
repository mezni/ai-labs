from dataclasses import dataclass


@dataclass(frozen=True)
class SpanStatus:
    value: str

    def __post_init__(self) -> None:
        if not self.value.strip():
            raise ValueError("Span status cannot be empty.")


SUCCESS = SpanStatus("success")
ERROR = SpanStatus("error")


class SpanStatuses:
    SUCCESS = SUCCESS
    ERROR = ERROR
