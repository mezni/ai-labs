"""Evaluation dataset: questions grounded in the policy corpus.

Each entry maps a question to (a) the reference answer and (b) the expected
source document (relative to the corpus dir) whose chunks must be retrieved
for the question to be answerable.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml

EVAL_QUESTIONS: list[dict[str, str]] = [
    {
        "question": "What are the eligibility requirements to upgrade a device?",
        "expected_answer": (
            "The device must be on the account for at least 12 months (or 50% of the "
            "installment term), there must be no more than 2 unpaid installments, the "
            "account must be Active, and the customer must not have exceeded device "
            "financing limits."
        ),
        "expected_source": "device_upgrade_guidelines.md",
    },
    {
        "question": (
            "What is the return window for a new device and how much is the "
            "restocking fee?"
        ),
        "expected_answer": (
            "New devices can be returned within 14 days. A $35 restocking fee applies "
            "unless the device is dead on arrival, in which case the fee is waived."
        ),
        "expected_source": "returns_refunds_policy.md",
    },
    {
        "question": "How many points are needed to reach Platinum tier in Aether Rewards?",
        "expected_answer": (
            "15,000 points. Platinum members earn 2 points per $1, receive a free "
            "device upgrade once per year, and get concierge support."
        ),
        "expected_source": "loyalty_rewards_program.md",
    },
    {
        "question": "What does the international roaming Day Pass cost and include?",
        "expected_answer": (
            "The Day Pass costs $15 per day and includes 500 MB of high-speed data "
            "plus unlimited in-country calls and SMS for 24 hours."
        ),
        "expected_source": "international_roaming_policy.md",
    },
    {
        "question": "Does Aether Wireless sell customer data to third parties?",
        "expected_answer": (
            "No. Aether Wireless does not sell customer personal data to third parties; "
            "data is shared only with service providers under contract."
        ),
        "expected_source": "privacy_data_protection_policy.md",
    },
    {
        "question": "What happens after a customer accepts a trade-in quote?",
        "expected_answer": (
            "The trade-in credit is applied immediately to the new device purchase. The "
            "old device is collected in-store or shipped via prepaid label within a "
            "7-day window, then sent to the refurbishment facility for final inspection; "
            "if the inspection changes the condition, the customer is billed or credited "
            "the difference."
        ),
        "expected_source": "device_trade_in_process.md",
    },
]


def load_dataset(path: str | Path | None = None) -> list[dict[str, Any]]:
    """Load the evaluation dataset from a YAML file, or return the built-in one."""
    if path is None:
        return list(EVAL_QUESTIONS)
    config_path = Path(path)
    with open(config_path, "r", encoding="utf-8") as handle:
        data = yaml.safe_load(handle)
    if not isinstance(data, list):
        raise ValueError(f"Dataset {config_path} must be a YAML list of entries")
    return data