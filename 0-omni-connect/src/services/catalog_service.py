"""
Product catalog lookup service.

Loads the plan and device catalogs from
data/business_data/product_catalog.json and exposes ID-based lookups for
the AI engine. Mirrors the bootcamp business-data service pattern.
"""
import json
from pathlib import Path
from typing import Any, Dict


def load_business_data(filename: str) -> Dict[str, Any]:
    """Load a business data JSON file from data/business_data/."""
    path = Path("data/business_data") / filename
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def get_plan(plan_id: str) -> Dict[str, Any]:
    """
    Look up a rate plan (allowances, monthly price, features) by plan_id.
    """
    data = load_business_data("product_catalog.json")
    for plan in data.get("plans", []):
        if plan.get("plan_id") == plan_id:
            return plan
    return {"error": "Plan not found", "plan_id": plan_id}


def get_device(device_id: str) -> Dict[str, Any]:
    """
    Look up a device (brand, model, storage, retail price) by device_id.
    """
    data = load_business_data("product_catalog.json")
    for device in data.get("devices", []):
        if device.get("device_id") == device_id:
            return device
    return {"error": "Device not found", "device_id": device_id}