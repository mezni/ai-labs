from dataclasses import dataclass


@dataclass(frozen=True)
class ExponentialBackoff:
    initial_delay_seconds: float = 1.0
    multiplier: float = 2.0
    max_delay_seconds: float = 30.0

    def __post_init__(self) -> None:
        if self.initial_delay_seconds < 0:
            raise ValueError(
                "initial_delay_seconds cannot be negative."
            )

        if self.multiplier < 1:
            raise ValueError(
                "multiplier must be at least 1."
            )

        if self.max_delay_seconds < 0:
            raise ValueError(
                "max_delay_seconds cannot be negative."
            )

        if self.max_delay_seconds < self.initial_delay_seconds:
            raise ValueError(
                "max_delay_seconds cannot be less than "
                "initial_delay_seconds."
            )

    def calculate_delay(self, attempt: int) -> float:
        if attempt < 1:
            raise ValueError("attempt must be at least 1.")

        delay = (
            self.initial_delay_seconds
            * (self.multiplier ** (attempt - 1))
        )

        return min(
            delay,
            self.max_delay_seconds,
        )
