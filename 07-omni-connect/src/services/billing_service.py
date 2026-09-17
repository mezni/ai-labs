"""
Billing account lookup service.

Loads billing records from data/business_data/billing_records.json and
exposes ID-based lookups for the AI engine. Mirrors the bootcamp
business-data service pattern.
"""
import json
from pathlib import Path
from typing import Any, Dict


def load_business_data(filename: str) -> Dict[str, Any]:
    """Load a business data JSON file from data/business_data/."""
    path = Path("data/business_data") / filename
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def get_billing_account(billing_account_id: str) -> Dict[str, Any]:
    """
    Look up a billing account (device, current balance, autopay, payment
    methods, invoice history) by its billing_account_id.
    """
    data = load_business_data("billing_records.json")
    for record in data:
        if record.get("billing_account_id") == billing_account_id:
            return record
    return {"error": "Billing account not found", "billing_account_id": billing_account_id}