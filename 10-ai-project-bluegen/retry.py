from dataclasses import dataclass

from failure import Failure


@dataclass(frozen=True)
class RetryDecision:
    retry: bool
    next_attempt: int | None
    reason: str

    def __post_init__(self) -> None:
        if not self.reason.strip():
            raise ValueError("reason cannot be empty.")

        if self.retry and self.next_attempt is None:
            raise ValueError(
                "next_attempt is required when retry is True."
            )

        if not self.retry and self.next_attempt is not None:
            raise ValueError(
                "next_attempt must be None when retry is False."
            )


@dataclass(frozen=True)
class RetryPolicy:
    max_attempts: int = 3

    def __post_init__(self) -> None:
        if self.max_attempts < 1:
            raise ValueError("max_attempts must be at least 1.")

    def decide(
        self,
        failure: Failure,
        attempt: int,
    ) -> RetryDecision:
        if attempt < 1:
            raise ValueError("attempt must be at least 1.")

        if not failure.retryable:
            return RetryDecision(
                retry=False,
                next_attempt=None,
                reason="Failure is not retryable.",
            )

        if attempt >= self.max_attempts:
            return RetryDecision(
                retry=False,
                next_attempt=None,
                reason="Maximum retry attempts reached.",
            )

        return RetryDecision(
            retry=True,
            next_attempt=attempt + 1,
            reason="Failure is retryable and attempts remain.",
        )
