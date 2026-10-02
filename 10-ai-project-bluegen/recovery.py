from dataclasses import dataclass
from enum import Enum

from errors import ErrorCategories
from failure import Failure


class RecoveryAction(str, Enum):
    FAIL = "fail"
    FALLBACK = "fallback"
    ESCALATE = "escalate"


@dataclass(frozen=True)
class RecoveryDecision:
    action: RecoveryAction
    reason: str

    def __post_init__(self) -> None:
        if not self.reason.strip():
            raise ValueError("reason cannot be empty.")


@dataclass(frozen=True)
class RecoveryStrategy:
    def recover(
        self,
        failure: Failure,
    ) -> RecoveryDecision:
        if failure.category == ErrorCategories.TIMEOUT:
            return RecoveryDecision(
                action=RecoveryAction.FALLBACK,
                reason="Timeout failure requires fallback.",
            )

        if failure.category == ErrorCategories.LLM:
            return RecoveryDecision(
                action=RecoveryAction.ESCALATE,
                reason="LLM failure requires escalation.",
            )

        return RecoveryDecision(
            action=RecoveryAction.FAIL,
            reason="Failure cannot be recovered automatically.",
        )
