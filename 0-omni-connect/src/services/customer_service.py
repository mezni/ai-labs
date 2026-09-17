"""
Customer relationship (CRM) lookup service.

Loads customer records from data/business_data/crm_records.json and exposes
ID-based lookups for the AI engine. Mirrors the bootcamp business-data
service pattern.
"""
import json
from pathlib import Path
from typing import Any, Dict


def load_business_data(filename: str) -> Dict[str, Any]:
    """Load a business data JSON file from data/business_data/."""
    path = Path("data/business_data") / filename
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def get_customer(customer_id: str) -> Dict[str, Any]:
    """
    Look up a customer record (plan, eligible promotions, account status,
    lifetime value, contact) by customer_id.
    """
    data = load_business_data("crm_records.json")
    for record in data:
        if record.get("customer_id") == customer_id:
            return record
    return {"error": "Customer not found", "customer_id": customer_id}