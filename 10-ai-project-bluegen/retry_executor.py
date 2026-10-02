from collections.abc import Callable
from dataclasses import dataclass, field
import logging
import time
from typing import Generic, TypeVar

from backoff import ExponentialBackoff
from exceptions import ProjectGenerationError
from failure import Failure
from logger import StructuredLogger
from recovery import RecoveryAction, RecoveryStrategy
from retry import RetryPolicy

T = TypeVar("T")


@dataclass(frozen=True)
class RetryOutcome(Generic[T]):
    result: T
    attempts: int
    recovery_action: RecoveryAction | None
    failure: Failure | None


@dataclass(frozen=True)
class RetryExecutor:
    retry_policy: RetryPolicy = field(
        default_factory=RetryPolicy
    )
    backoff: ExponentialBackoff = field(
        default_factory=ExponentialBackoff
    )
    recovery_strategy: RecoveryStrategy = field(
        default_factory=RecoveryStrategy
    )
    sleep: Callable[[float], None] = time.sleep
    logger: StructuredLogger = field(
        default_factory=lambda: StructuredLogger(
            logging.getLogger(__name__)
        ),
        compare=False,
        repr=False,
    )

    def execute(
        self,
        operation: Callable[[], T],
        fallback: Callable[[], T] | None = None,
        request_id: str | None = None,
    ) -> RetryOutcome[T]:
        attempt = 1

        last_failure: Failure | None = None
        last_error: ProjectGenerationError | None = None

        while True:
            try:
                result = operation()

            except ProjectGenerationError as error:
                failure = error.failure

                if failure is None:
                    self.logger.error(
                        "Unclassified generation error; "
                        "not retrying",
                        request_id=request_id,
                        operation="retry_execute",
                    )

                    raise

                last_failure = failure
                last_error = error

                decision = self.retry_policy.decide(
                    failure,
                    attempt,
                )

                self.logger.info(
                    "Retry decision: "
                    f"retry={decision.retry}; "
                    f"{decision.reason}",
                    request_id=request_id,
                    operation="retry_execute",
                )

                if not decision.retry:
                    break

                delay = self.backoff.calculate_delay(attempt)

                self.logger.info(
                    f"Waiting {delay:.2f}s before attempt "
                    f"{decision.next_attempt}",
                    request_id=request_id,
                    operation="retry_execute",
                )

                self.sleep(delay)

                attempt = decision.next_attempt or attempt + 1

                continue

            return RetryOutcome(
                result=result,
                attempts=attempt,
                recovery_action=None,
                failure=None,
            )

        if last_error is None or last_failure is None:
            raise RuntimeError(
                "Retry loop ended without a recorded failure."
            )

        recovery = self.recovery_strategy.recover(last_failure)

        self.logger.info(
            f"Recovery decision: {recovery.action.value}; "
            f"{recovery.reason}",
            request_id=request_id,
            operation="retry_execute",
        )

        if recovery.action == RecoveryAction.FALLBACK:
            if fallback is not None:
                return RetryOutcome(
                    result=fallback(),
                    attempts=attempt,
                    recovery_action=RecoveryAction.FALLBACK,
                    failure=last_failure,
                )

            self.logger.error(
                "Fallback requested but no fallback "
                "operation was provided",
                request_id=request_id,
                operation="retry_execute",
            )

        raise last_error
