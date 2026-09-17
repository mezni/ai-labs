"""
Business Data Generator
Generates the Category A structured business data documented in docs/data.md:

    product_catalog.json (plans + devices)
    promotions.json
    crm_records.json + billing_records.json

The deterministic catalogs (plans, devices, promotions) are embedded below as
constants — they mirror the ground-truth tables in docs/data.md Categories A/B.
The variable, person-level data (names, emails, phones, companies, creation
timestamps, payment details) is synthesized with the `faker` package while
honoring the business rules documented in docs/data.md Category B (15-day due
window, ~10% regulatory taxes & fees, autopay on by default, up to 2 eligible
offers per interaction matching the account status, etc.).

Output mirrors docs/data.md Category A and is written to:
    data/business_data/
        - product_catalog.json
        - promotions.json
        - crm_records.json
        - billing_records.json

Note: `usage_telemetry.json` (A.5) is documented as *pending* in docs/data.md and
is therefore intentionally not generated here.

Run:  python scripts/business_data_generator.py [--count N] [--seed S]
"""

import argparse
import json
import random
import secrets
from dataclasses import asdict, dataclass
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import List, Optional

from faker import Faker

# ==============================================================================
# 0. OUTPUT PATH
# ==============================================================================

ROOT = Path(__file__).resolve().parent.parent
OUTPUT_DIR = ROOT / "data" / "business_data"

# ==============================================================================
# 1. CATEGORY A SCHEMAS
# ==============================================================================


@dataclass
class Plan:
    plan_id: str
    name: str
    data_allowance_gb: int        # 0 denotes unlimited
    talk_minutes: int             # 0 denotes unlimited
    text_messages: int            # 0 denotes unlimited
    monthly_price: float
    hotspot_included: bool
    international_calling: bool


@dataclass
class Device:
    device_id: str
    brand: str
    model: str
    storage_gb: int
    color: str
    retail_price: float
    release_year: int


@dataclass
class Promotion:
    promotion_id: str
    title: str
    description: str
    promo_type: str               # PLAN_OFFER | DEVICE_PROMO | BUNDLE
    target_plan_id: Optional[str]
    target_device_id: Optional[str]
    discount_amount: float        # monthly for plan offers, one-time otherwise
    eligible_account_statuses: List[str]
    valid_until: str              # ISO 8601, UTC


# ==============================================================================
# 2. DETERMINISTIC CATALOGS (mirror docs/data.md A.3 / A.4 / B.11)
# ==============================================================================

PLANS = [
    Plan("PLAN-001", "Essential", 5, 500, 500, 40.00, False, False),
    Plan("PLAN-002", "Standard", 20, 1_000, 1_000, 60.00, True, False),
    Plan("PLAN-003", "Premium", 50, 0, 0, 80.00, True, True),
    Plan("PLAN-004", "Family", 100, 0, 0, 120.00, True, True),
    Plan("PLAN-005", "Unlimited", 0, 0, 0, 90.00, True, True),
]

DEVICES = [
    Device("DEV-1001", "Apple", "iPhone 15", 128, "Black", 799.00, 2023),
    Device("DEV-1002", "Apple", "iPhone 15 Pro", 256, "Natural Titanium", 1_099.00, 2023),
    Device("DEV-1003", "Samsung", "Galaxy S24", 128, "Onyx Black", 799.00, 2024),
    Device("DEV-1004", "Samsung", "Galaxy S24 Ultra", 512, "Titanium Gray", 1_199.00, 2024),
    Device("DEV-1005", "Google", "Pixel 8", 128, "Obsidian", 699.00, 2023),
]

PROMOTIONS = [
    Promotion("PROMO-0001", "Premium Upgrade Discount",
              "$10/mo off the Premium plan for the first 12 months.",
              "PLAN_OFFER", "PLAN-003", None, 10.0,
              ["Active", "Prospect"], "2026-12-03T00:00:00+00:00"),
    Promotion("PROMO-0002", "Family Bundle Deal",
              "$15/mo off the Family plan with 2+ lines.",
              "PLAN_OFFER", "PLAN-004", None, 15.0,
              ["Active"], "2026-12-03T00:00:00+00:00"),
    Promotion("PROMO-0003", "New Device Trade-In Bonus",
              "$200 off any flagship device with a qualifying trade-in.",
              "DEVICE_PROMO", None, None, 200.0,
              ["Active", "Prospect"], "2026-12-03T00:00:00+00:00"),
    Promotion("PROMO-0004", "Premium + Pixel Bundle",
              "Premium plan bundled with a Pixel 8 at $150 off.",
              "BUNDLE", "PLAN-003", "DEV-1005", 150.0,
              ["Active"], "2026-12-03T00:00:00+00:00"),
    Promotion("PROMO-0005", "Win-Back Unlimited Offer",
              "Unlimited plan at $75/mo to win back delinquent accounts.",
              "PLAN_OFFER", "PLAN-005", None, 15.0,
              ["Delinquent"], "2026-12-03T00:00:00+00:00"),
]

# ==============================================================================
# 3. SYNTHETIC GENERATORS (faker-backed)
# ==============================================================================

PAID_OR_OVERDUE = ["PAID", "PAID", "PAID", "OVERDUE"]


def _rand_id(prefix: str, digits: int, used: set) -> str:
    candidate = f"{prefix}-{secrets.randbelow(10 ** digits):0{digits}d}"
    while candidate in used:
        candidate = f"{prefix}-{secrets.randbelow(10 ** digits):0{digits}d}"
    used.add(candidate)
    return candidate


def _billing_history(customer_id: str, billing_account_id: str,
                     created_at: datetime, plan: Plan, used_ids: set) -> List[dict]:
    invoices = []
    now = datetime.now(timezone.utc)
    for month_offset in range(6, 0, -1):  # oldest → newest
        issue_date = now - timedelta(days=30 * month_offset)
        if issue_date < created_at:
            issue_date = created_at + timedelta(days=1)
        due_date = issue_date + timedelta(days=15)
        plan_fee = plan.monthly_price
        taxes = round(plan_fee * 0.10, 2)
        invoices.append(
            {
                "invoice_id": _rand_id("INV", 8, used_ids),
                "customer_id": customer_id,
                "billing_account_id": billing_account_id,
                "plan_id": plan.plan_id,
                "issue_date": issue_date.isoformat(),
                "due_date": due_date.isoformat(),
                "total_amount": round(plan_fee + taxes, 2),
                "status": random.choice(PAID_OR_OVERDUE),
                "line_items": [
                    {"description": f"{plan.name} — Monthly Rate Plan", "amount": plan_fee},
                    {"description": "Regulatory Taxes & Fees", "amount": taxes},
                ],
            }
        )
    return invoices


def _generate_pair(fake: Faker, plans: List[Plan], devices: List[Device],
                   promotions: List[Promotion], used_ids: set) -> tuple[dict, dict]:
    customer_id = _rand_id("CUST", 6, used_ids)
    billing_account_id = _rand_id("BILL", 6, used_ids)

    plan = random.choice(plans)
    device = random.choice(devices)

    created_at = fake.date_time_between(start_date="-365d", end_date="now", tzinfo=timezone.utc)

    invoices = _billing_history(customer_id, billing_account_id, created_at, plan, used_ids)
    lifetime_value = round(sum(inv["total_amount"] for inv in invoices), 2)

    account_status = random.choice(["Active", "Delinquent", "Prospect"]) if random.random() < 0.85 else "Active"
    eligible_promotion_ids = [
        promo.promotion_id
        for promo in promotions
        if account_status in promo.eligible_account_statuses
    ]
    if account_status == "Delinquent":
        eligible_promotion_ids = ["PROMO-0005"]
    elif len(eligible_promotion_ids) > 2:
        eligible_promotion_ids = random.sample(eligible_promotion_ids, 2)

    billing_account = {
        "billing_account_id": billing_account_id,
        "customer_id": customer_id,
        "device_id": device.device_id,
        "currency": "USD",
        "current_balance": 0.0 if invoices[-1]["status"] == "PAID" else invoices[-1]["total_amount"],
        "autopay_enabled": random.random() < 0.75,
        "payment_methods": [
            {
                "payment_method_id": _rand_id("PM", 6, used_ids),
                "type": random.choice(["CREDIT_CARD", "DEBIT_CARD", "ACH"]),
                "last_four": fake.numerify("####"),
                "is_default": True,
            }
        ],
        "invoices": invoices,
    }

    crm_record = {
        "customer_id": customer_id,
        "billing_account_id": billing_account_id,
        "current_plan_id": plan.plan_id,
        "eligible_promotion_ids": sorted(eligible_promotion_ids),
        "contact": {
            "first_name": fake.first_name(),
            "last_name": fake.last_name(),
            "email": fake.safe_email(),
            "phone": fake.phone_number(),
            "company": fake.company(),
        },
        "account_status": account_status,
        "lifetime_value": lifetime_value,
        "created_at": created_at.isoformat(),
    }

    return crm_record, billing_account


# ==============================================================================
# 4. EXECUTION
# ==============================================================================


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate Category A business data")
    parser.add_argument("--count", type=int, default=10, help="number of CRM + billing pairs (default: 10)")
    parser.add_argument("--seed", type=int, default=None, help="random seed for reproducibility")
    args = parser.parse_args()

    fake = Faker()
    if args.seed is not None:
        Faker.seed(args.seed)
        random.seed(args.seed)

    used_ids: set = set()
    crm_records, billing_records = [], []
    for _ in range(args.count):
        crm, billing = _generate_pair(fake, PLANS, DEVICES, PROMOTIONS, used_ids)
        crm_records.append(crm)
        billing_records.append(billing)

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    outputs = {
        "product_catalog.json": {"plans": [asdict(p) for p in PLANS],
                                 "devices": [asdict(d) for d in DEVICES]},
        "promotions.json": [asdict(p) for p in PROMOTIONS],
        "crm_records.json": crm_records,
        "billing_records.json": billing_records,
    }

    print(f"Successfully generated datasets under '{OUTPUT_DIR.resolve()}':")
    for filename, payload in outputs.items():
        path = OUTPUT_DIR / filename
        path.write_text(json.dumps(payload, indent=2), encoding="utf-8")
        print(f" - {path}")


if __name__ == "__main__":
    main()