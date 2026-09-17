"""
Customer Profile Agent - analyzes a customer's usage patterns, plan details,
and line constraints for the omni-connect retail copilot.

Adapted from the bootcamp customer_profile_agent.py.txt pattern: a single
specialist agent that pulls the latest business data for a customer, formats
it into the customer_profile_prompt, and asks the LLM for a concise
professional summary. No recommendation is made here - that is another
agent's job.
"""
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, Optional

from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.services import billing_service, catalog_service, customer_service
from src.utils.logger import setup_logger

logger = setup_logger(__name__)


def _format_tenure(created_at: str) -> str:
    """Render account age as a short human-readable string."""
    created = datetime.fromisoformat(created_at)
    now = datetime.now(timezone.utc)
    years = (now - created).days / 365.25
    if years >= 1:
        return f"{max(1, round(years))} Years"
    months = max(1, round((now - created).days / 30.44))
    return f"{months} Months"


def _billing_summary(billing: Dict[str, Any]) -> str:
    """Condense a billing account into the fields the prompt template needs."""
    if "error" in billing:
        return "unavailable"
    invoices = billing.get("invoices", [])
    if invoices:
        avg = sum(inv["total_amount"] for inv in invoices) / len(invoices)
        overdue = sum(1 for inv in invoices if inv["status"] == "OVERDUE")
        reliability = "Overdue" if overdue else "Current"
    else:
        avg, overdue, reliability = 0.0, 0, "No history"
    return (
        f"monthly avg ${avg:.2f}, {reliability} ({overdue} overdue of {len(invoices)}), "
        f"autopay {'on' if billing.get('autopay_enabled') else 'off'}, "
        f"balance ${billing.get('current_balance', 0):.2f}"
    )


def _usage_telemetry(customer_id: str) -> Optional[Dict[str, Any]]:
    """Load usage_telemetry.json if present; 0 telemetry is pending in docs."""
    path = Path("data/business_data/usage_telemetry.json")
    if not path.exists():
        return None
    data = json.loads(path.read_text(encoding="utf-8"))
    records = data if isinstance(data, list) else data.get("records", [])
    return next((r for r in records if r.get("customer_id") == customer_id), None)


class CustomerProfileAgent:
    """Analyzes a customer's usage patterns, plan details, and line constraints."""

    def __init__(self, llm_client: Optional[LLMClient] = None,
                 prompt_manager: Optional[PromptManager] = None) -> None:
        self.llm = llm_client or LLMClient()
        self.prompts = prompt_manager or PromptManager()

    def analyze(self, customer_id: str, context: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Return a customer-profile summary for the given customer_id."""
        customer = customer_service.get_customer(customer_id)
        if "error" in customer:
            return {"agent": "customer_profile", "error": customer["error"], "customer_id": customer_id}

        billing = billing_service.get_billing_account(customer["billing_account_id"])
        plan = catalog_service.get_plan(customer["current_plan_id"])
        device = (
            catalog_service.get_device(billing["device_id"])
            if "device_id" in billing else {"error": "no device on billing account"}
        )

        contact = customer.get("contact", {})
        customer_name = f"{contact.get('first_name')} {contact.get('last_name')}".strip()
        plan_text = (
            f"{plan.get('name')} (${plan.get('monthly_price', 0):.2f}/mo, "
            f"{plan.get('data_allowance_gb')} GB, {plan.get('talk_minutes')} min, "
            f"{plan.get('text_messages')} SMS, hotspot "
            f"{'yes' if plan.get('hotspot_included') else 'no'})"
            if "error" not in plan else "unavailable"
        )
        device_text = (
            f"{device.get('brand')} {device.get('model')} ({device.get('storage_gb')} GB)"
            if "error" not in device else "unavailable"
        )
        telemetry = _usage_telemetry(customer_id)
        telemetry_text = json.dumps(telemetry) if telemetry else "no telemetry on record"

        prompt = self.prompts.get_prompt("customer_profile_prompt")
        user_message = self.prompts.format_prompt(
            prompt.get("user_template", ""),
            customer_id=customer.get("customer_id"),
            customer_name=customer_name,
            account_status=customer.get("account_status"),
            tenure=_format_tenure(customer["created_at"]),
            lifetime_value=f"${customer.get('lifetime_value', 0):,.2f}",
            current_plan=plan_text,
            device=device_text,
            billing_summary=_billing_summary(billing),
            eligible_promotions=", ".join(customer.get("eligible_promotion_ids", [])) or "none",
            usage_telemetry=telemetry_text,
        )
        summary = self.llm.generate([
            {"role": "system", "content": prompt.get("system", "")},
            {"role": "user", "content": user_message},
        ])

        return {
            "agent": "customer_profile",
            "customer_id": customer_id,
            "customer_name": customer_name,
            "summary": summary,
            "context": {
                "plan": plan_text,
                "device": device_text,
                "eligible_promotion_ids": customer.get("eligible_promotion_ids", []),
            },
        }