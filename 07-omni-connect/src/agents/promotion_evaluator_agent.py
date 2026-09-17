"""
Promotion Evaluator Agent - evaluates cross-compatibility between a
customer's profile and promotion policy requirements.

Adapted from the bootcamp compliance_agent.py.txt pattern: runs a small,
deterministic set of eligibility checks (account status, plan/device match,
offer expiry, offer-cap / stacking rules) per candidate promotion, then asks
the LLM to narrate the resulting eligibility statement from the
compliance_prompt template.

Candidate promotions default to the customer's eligible_promotion_ids from
the CRM record, so the agent cross-checks what the profile suggests against
the actual policy requirements.
"""
from datetime import datetime, timezone
from typing import Any, Dict, List, Optional

from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.services import billing_service, catalog_service, customer_service, promotion_service
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

OFFER_CAP_PER_INTERACTION = 2
WIN_BACK_EXCLUSIVE = "PROMO-0005"  # win-back is exclusive to delinquent accounts


class PromotionEvaluatorAgent:
    """Checks each candidate promotion against the customer profile + policy."""

    def __init__(self, llm_client: Optional[LLMClient] = None,
                 prompt_manager: Optional[PromptManager] = None) -> None:
        self.llm = llm_client or LLMClient()
        self.prompts = prompt_manager or PromptManager()

    @staticmethod
    def _check(promo: Dict[str, Any], customer: Dict[str, Any],
               device: Dict[str, Any]) -> tuple[List[str], List[str]]:
        """Return (blockers, notes) for a promotion vs the customer profile.

        Blockers make the offer not usable as-is (bad account status, expired,
        win-back on a non-delinquent account). Notes are conditions for the rep
        to action (target plan/device differ from what the customer holds) —
        plan offers exist precisely to move a customer onto the target plan, so
        a target-plan mismatch is informational, not a hard stop.
        """
        blockers: List[str] = []
        notes: List[str] = []

        statuses = promo.get("eligible_account_statuses", [])
        if customer.get("account_status") not in statuses:
            blockers.append(
                f"account status '{customer.get('account_status')}' not in "
                f"eligible {statuses}"
            )
        valid_until = promo.get("valid_until")
        if valid_until and datetime.fromisoformat(valid_until) < datetime.now(timezone.utc):
            blockers.append(f"offer expired {valid_until}")
        if promo.get("promotion_id") == WIN_BACK_EXCLUSIVE and customer.get("account_status") != "Delinquent":
            blockers.append("win-back offer is exclusive to delinquent accounts")

        target_plan = promo.get("target_plan_id")
        if target_plan and target_plan != customer.get("current_plan_id"):
            notes.append(f"target plan {target_plan} (customer on {customer.get('current_plan_id')})")
        target_device = promo.get("target_device_id")
        if target_device and target_device != device.get("device_id"):
            notes.append(f"target device {target_device} (customer has {device.get('device_id')})")
        return blockers, notes

    def evaluate(
        self,
        customer_id: str,
        promotion_ids: Optional[List[str]] = None,
        context: Optional[Dict[str, Any]] = None,
    ) -> Dict[str, Any]:
        """Return per-promotion eligibility verdicts and an LLM narrative."""
        customer = customer_service.get_customer(customer_id)
        if "error" in customer:
            return {"agent": "promotion_evaluator", "error": customer["error"], "customer_id": customer_id}

        billing = billing_service.get_billing_account(customer["billing_account_id"])
        plan = catalog_service.get_plan(customer["current_plan_id"])
        device = (
            catalog_service.get_device(billing["device_id"])
            if "device_id" in billing else {"error": "no device on billing account"}
        )

        candidates = promotion_ids or customer.get("eligible_promotion_ids", [])
        promotions = promotion_service.load_business_data("promotions.json")
        by_id = {promo["promotion_id"]: promo for promo in promotions}

        checks = []
        evaluations = []
        for promo_id in candidates:
            promo = by_id.get(promo_id)
            if promo is None:
                eval_entry = {"promotion_id": promo_id, "title": "?", "status": "Not Eligible",
                              "reasons": ["unknown promotion"]}
            else:
                blockers, notes = self._check(promo, customer, device)
                status = "Eligible" if not blockers else "Not Eligible"
                eval_entry = {
                    "promotion_id": promo_id,
                    "title": promo.get("title"),
                    "status": status,
                    "reasons": blockers,
                    "notes": notes,
                }
            evaluations.append(eval_entry)
            if eval_entry["status"] == "Eligible":
                checks.append(
                    f"- {eval_entry['promotion_id']} {eval_entry['title']} -> Eligible"
                    + (" (note: " + "; ".join(eval_entry["notes"]) + ")" if eval_entry["notes"] else "")
                )
            else:
                checks.append(
                    f"- {eval_entry['promotion_id']} {eval_entry['title']} -> "
                    f"Not Eligible: {'; '.join(eval_entry['reasons'])}"
                )

        eligible = [e["promotion_id"] for e in evaluations if e["status"] == "Eligible"]
        eligible = eligible[:OFFER_CAP_PER_INTERACTION]

        contact = customer.get("contact", {})
        customer_name = f"{contact.get('first_name')} {contact.get('last_name')}".strip()
        prompt = self.prompts.get_prompt("compliance_prompt")
        user_message = self.prompts.format_prompt(
            prompt.get("user_template", ""),
            customer_name=customer_name,
            customer_id=customer.get("customer_id"),
            account_status=customer.get("account_status"),
            current_plan=plan.get("name") if "error" not in plan else "unavailable",
            device=f"{device.get('brand')} {device.get('model')}" if "error" not in device else "unavailable",
            checks="\n".join(checks) if checks else "No candidate promotions.",
            eligible_promotions=", ".join(eligible) or "none",
        )
        summary = self.llm.generate([
            {"role": "system", "content": prompt.get("system", "")},
            {"role": "user", "content": user_message},
        ])

        return {
            "agent": "promotion_evaluator",
            "customer_id": customer_id,
            "status": "Eligible" if eligible else "Not Eligible",
            "evaluations": evaluations,
            "eligible_promotion_ids": eligible,
            "summary": summary,
        }