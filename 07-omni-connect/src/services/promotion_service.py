"""
Promotion lookup service.

Loads the promotional offer catalog from data/business_data/promotions.json
and exposes ID-based lookups for the AI engine. Mirrors the bootcamp
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


def get_promotion(promotion_id: str) -> Dict[str, Any]:
    """
    Look up a promotion (type, target plan/device, discount, eligibility,
    validity) by its promotion_id.
    """
    data = load_business_data("promotions.json")
    for promotion in data:
        if promotion.get("promotion_id") == promotion_id:
            return promotion
    return {"error": "Promotion not found", "promotion_id": promotion_id}