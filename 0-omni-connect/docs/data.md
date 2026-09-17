# Retail Data Sources & Schema Reference

This document defines the canonical data files that feed the AI Retail Copilot platform. Structured sources are single JSON payloads (or JSON-lines batches) and serve as ground truth for the AI engine's reasoning, recommendations, and eligibility checks. Policy sources are authored markdown documents ingested by the AI engine as reference knowledge. Operational sources are runtime event logs produced by the platform for monitoring, compliance, and improvement.

Data sources are organized into categories. Sections belonging to the same category are marked with the category letter.

### Category Index

| Category | Title | Files |
|---|---|---|
| A | Business Data — Customer, Billing, Product & Promotions | `crm_records.json`, `billing_records.json`, `product_catalog.json`, `promotions.json`, `usage_telemetry.json` *(pending)* |
| B | Knowledge Base — Policy & Policy-Driven Processes | `billing_invoice_policy.md.txt`, `data_plan_terms_conditions.md.txt`, `device_protection_policy.md.txt`, `device_trade_in_process.md.txt`, `device_upgrade_guidelines.md.txt`, `new_line_activation_process.md.txt`, `payment_eligibility_terms.md.txt`, `plan_upgrade_policy.md.txt`, `port_in_policy.md.txt`, `postpaid_plan_guidelines.md.txt`, `promotions_eligibility_terms.md.txt`, `returns_refunds_policy.md.txt` |
| C | Operational Data — Sessions, Recommendations, Performance & Audits | `rep_session_action.json`, `copilot_recommendation.json`, `system_telemetry.json`, `guardrail_audit.json`, `rep_feedback.json` |

---

## Category A — Business Data (Customer, Billing, Product & Promotions)

Business sources describing who the customer is, how they are billed, what plans/devices are sellable, and the promotional rules that govern offers. These are the primary ground-truth records the AI engine joins against Category B policy rules to generate recommendations.

### A.1 CRM Records — `crm_records.json`

**Purpose:** Provides the core customer context — account identity, current plan, account status, and eligible promotions.

### Record: Customer

| Field | Type | Description |
|---|---|---|
| `customer_id` | string | Unique customer identifier (e.g. `CUST-939042`). |
| `billing_account_id` | string | Reference to the billing account (see `billing_records.json`). |
| `current_plan_id` | string | Active plan code (see `product_catalog.json`). |
| `eligible_promotion_ids` | array | Promotion codes the customer is eligible for (see `promotions.json`). |
| `contact` | object | Nested contact block: `first_name`, `last_name`, `email`, `phone`, `company`. |
| `account_status` | string | `Active`, `Prospect`, or `Delinquent` (see B.1). |
| `lifetime_value` | number | Computed lifetime value (currency). |
| `created_at` | string | Account creation timestamp (ISO 8601, UTC). |

### Example

```json
{
  "customer_id": "CUST-939042",
  "billing_account_id": "BILL-915552",
  "current_plan_id": "PLAN-002",
  "eligible_promotion_ids": ["PROMO-0001", "PROMO-0003"],
  "contact": {
    "first_name": "Anthony",
    "last_name": "Russell",
    "email": "erin52@example.net",
    "phone": "505.226.4719x71892",
    "company": "Robinson, Fitzgerald and Rodriguez"
  },
  "account_status": "Prospect",
  "lifetime_value": 396.0,
  "created_at": "2026-04-21T21:01:04.632529+00:00"
}
```

### Data (10 records)

| customer_id | billing_account_id | plan | account_status | eligible promotions | lifetime_value |
|---|---|---|---|---|---|
| `CUST-939042` | `BILL-915552` | PLAN-002 · Standard | Prospect | PROMO-0001, PROMO-0003 | 396.00 |
| `CUST-365029` | `BILL-145294` | PLAN-003 · Premium | Prospect | PROMO-0001, PROMO-0003 | 528.00 |
| `CUST-718907` | `BILL-708411` | PLAN-005 · Unlimited | Active | PROMO-0001, PROMO-0002 | 594.00 |
| `CUST-693645` | `BILL-686986` | PLAN-005 · Unlimited | Prospect | PROMO-0001, PROMO-0003 | 594.00 |
| `CUST-575307` | `BILL-526322` | PLAN-001 · Essential | Delinquent | PROMO-0005 | 264.00 |
| `CUST-276508` | `BILL-446892` | PLAN-004 · Family | Active | PROMO-0001, PROMO-0002 | 792.00 |
| `CUST-319374` | `BILL-398660` | PLAN-002 · Standard | Active | PROMO-0001, PROMO-0002 | 396.00 |
| `CUST-285821` | `BILL-401432` | PLAN-004 · Family | Active | PROMO-0001, PROMO-0002 | 792.00 |
| `CUST-260702` | `BILL-761991` | PLAN-004 · Family | Active | PROMO-0001, PROMO-0002 | 792.00 |
| `CUST-861521` | `BILL-606294` | PLAN-002 · Standard | Prospect | PROMO-0001, PROMO-0003 | 396.00 |

---

### A.2 Billing Records — `billing_records.json`

**Purpose:** Provides account billing context — current balance, autopay, payment methods, and full invoice history.

### Record: BillingAccount

| Field | Type | Description |
|---|---|---|
| `billing_account_id` | string | Unique billing account identifier. |
| `customer_id` | string | Reference to the owning customer (see `crm_records.json`). |
| `device_id` | string | Device on the account (see `product_catalog.json`). |
| `currency` | string | Billing currency (e.g. `USD`). |
| `current_balance` | number | Outstanding balance (currency). |
| `autopay_enabled` | boolean | Whether autopay is active (see B.1 for the $5/month autopay discount). |
| `payment_methods` | array | One or more payment method objects (see below). |
| `invoices` | array | Invoice history (see below). |

### Record: PaymentMethod (element of `payment_methods`)

| Field | Type | Description |
|---|---|---|
| `payment_method_id` | string | Unique payment method identifier. |
| `type` | string | `CREDIT_CARD`, `DEBIT_CARD`, or `ACH` (see B.7). |
| `last_four` | string | Last four digits of the card/ACH account. |
| `is_default` | boolean | Whether this is the default method. |

### Record: Invoice (element of `invoices`)

| Field | Type | Description |
|---|---|---|
| `invoice_id` | string | Unique invoice identifier. |
| `customer_id` | string | Reference to the owning customer. |
| `billing_account_id` | string | Reference to the billing account. |
| `plan_id` | string | Plan billed for the cycle (see `product_catalog.json`). |
| `issue_date` | string | Invoice issue date (ISO 8601, UTC). |
| `due_date` | string | Due date, 15 days after issue (see B.1). |
| `total_amount` | number | Total = plan price + regulatory taxes & fees (~10%). |
| `status` | string | `PAID` or `OVERDUE`. |
| `line_items` | array | Billed line items (plan, taxes & fees, etc.). |

### Record: LineItem (element of `line_items`)

| Field | Type | Description |
|---|---|---|
| `description` | string | Line item description (e.g. "Standard — Monthly Rate Plan"). |
| `amount` | number | Line item amount (currency). |

### Example (first account, abbreviated invoice history)

```json
{
  "billing_account_id": "BILL-915552",
  "customer_id": "CUST-939042",
  "device_id": "DEV-1005",
  "currency": "USD",
  "current_balance": 0.0,
  "autopay_enabled": true,
  "payment_methods": [
    { "payment_method_id": "PM-584196", "type": "CREDIT_CARD", "last_four": "1449", "is_default": true }
  ],
  "invoices": [
    {
      "invoice_id": "INV-73183160",
      "customer_id": "CUST-939042",
      "billing_account_id": "BILL-915552",
      "plan_id": "PLAN-002",
      "issue_date": "2026-05-07T21:01:04.632543+00:00",
      "due_date": "2026-05-22T21:01:04.632543+00:00",
      "total_amount": 66.0,
      "status": "OVERDUE",
      "line_items": [
        { "description": "Standard — Monthly Rate Plan", "amount": 60.0 },
        { "description": "Regulatory Taxes & Fees", "amount": 6.0 }
      ]
    }
  ]
}
```

### Data summary (10 billing accounts)

| billing_account_id | customer_id | device | current_balance | autopay | default payment |
|---|---|---|---|---|---|
| `BILL-915552` | `CUST-939042` | DEV-1005 · Pixel 8 | 0.00 | true | CREDIT_CARD ···1449 |
| `BILL-145294` | `CUST-365029` | DEV-1004 · Galaxy S24 Ultra | 0.00 | true | CREDIT_CARD ···7914 |
| `BILL-708411` | `CUST-718907` | DEV-1003 · Galaxy S24 | 0.00 | true | CREDIT_CARD ···1387 |
| `BILL-686986` | `CUST-693645` | DEV-1005 · Pixel 8 | 0.00 | true | DEBIT_CARD ···4838 |
| `BILL-526322` | `CUST-575307` | DEV-1001 · iPhone 15 | 44.00 | true | ACH ···4534 |
| `BILL-446892` | `CUST-276508` | DEV-1005 · Pixel 8 | 132.00 | true | DEBIT_CARD ···6946 |
| `BILL-398660` | `CUST-319374` | DEV-1003 · Galaxy S24 | 0.00 | true | CREDIT_CARD ···6420 |
| `BILL-401432` | `CUST-285821` | DEV-1004 · Galaxy S24 Ultra | 0.00 | false | DEBIT_CARD ···5990 |
| `BILL-761991` | `CUST-260702` | DEV-1004 · Galaxy S24 Ultra | 0.00 | true | ACH ···3127 |
| `BILL-606294` | `CUST-861521` | DEV-1003 · Galaxy S24 | 66.00 | true | ACH ···7125 |

Note: invoice totals equal plan price + ~10% regulatory taxes & fees (Essential $44, Standard $66, Premium $88, Family $132, Unlimited $99).

---

### A.3 Product Catalog — `product_catalog.json`

**Purpose:** Ground truth for plan comparisons (allowances, features, pricing) and the device/hardware lineup.

### Record: Plan

| Field | Type | Description |
|---|---|---|
| `plan_id` | string | Unique plan code (e.g. `PLAN-001`). |
| `name` | string | Plan name (Essential, Standard, Premium, Family, Unlimited). |
| `data_allowance_gb` | integer | High-speed data allowance (GB); `0` = unlimited. |
| `talk_minutes` | integer | Included talk minutes; `0` = unlimited. |
| `text_messages` | integer | Included texts; `0` = unlimited. |
| `monthly_price` | number | Monthly plan price (currency). |
| `hotspot_included` | boolean | Whether mobile hotspot is included. |
| `international_calling` | boolean | Whether international calling is included. |

### Plans (5)

| plan_id | name | data_allowance_gb | talk_minutes | text_messages | monthly_price | hotspot | intl. calling |
|---|---|---|---|---|---|---|---|
| PLAN-001 | Essential | 5 | 500 | 500 | $40.00 | No | No |
| PLAN-002 | Standard | 20 | 1,000 | 1,000 | $60.00 | Yes | No |
| PLAN-003 | Premium | 50 | Unlimited | Unlimited | $80.00 | Yes | Yes |
| PLAN-004 | Family | 100 | Unlimited | Unlimited | $120.00 | Yes | Yes |
| PLAN-005 | Unlimited | Unlimited | Unlimited | Unlimited | $90.00 | Yes | Yes |

`talk_minutes`, `text_messages`, and `data_allowance_gb` of `0` denote unlimited. Matches the B.10 postpaid plan guidelines.

### Record: Device

| Field | Type | Description |
|---|---|---|
| `device_id` | string | Unique device SKU (e.g. `DEV-1001`). |
| `brand` | string | Manufacturer (Apple, Samsung, Google). |
| `model` | string | Model name. |
| `storage_gb` | integer | Storage capacity (GB). |
| `color` | string | Color variant. |
| `retail_price` | number | Unsubsidized retail price (currency). |
| `release_year` | integer | Year of release. |

### Devices (5)

| device_id | brand | model | storage_gb | color | retail_price | release_year |
|---|---|---|---|---|---|---|
| DEV-1001 | Apple | iPhone 15 | 128 | Black | $799.00 | 2023 |
| DEV-1002 | Apple | iPhone 15 Pro | 256 | Natural Titanium | $1,099.00 | 2023 |
| DEV-1003 | Samsung | Galaxy S24 | 128 | Onyx Black | $799.00 | 2024 |
| DEV-1004 | Samsung | Galaxy S24 Ultra | 512 | Titanium Gray | $1,199.00 | 2024 |
| DEV-1005 | Google | Pixel 8 | 128 | Obsidian | $699.00 | 2023 |

Retail prices match the B.5 device upgrade / installment financing table (0% APR when financed on Premium or Unlimited).

Store-level stock (`device_inventory.json`) and trade-in value bands are defined in the platform schema but not yet populated in the current data set.

---

### A.4 Promotions & Eligibility — `promotions.json`

**Purpose:** Deterministic promotional rules for the AI engine. Matches the B.11 offer catalog (PROMO-0001–PROMO-0005) and the `eligible_promotion_ids` assigned to customers in `crm_records.json`.

### Record: Promotion

| Field | Type | Description |
|---|---|---|
| `promotion_id` | string | Unique promotion code (e.g. `PROMO-0001`). |
| `title` | string | Offer title. |
| `description` | string | Human-readable offer description. |
| `promo_type` | string | `PLAN_OFFER`, `DEVICE_PROMO`, or `BUNDLE`. |
| `target_plan_id` | string | Plan the offer applies to (`plan_id` in `product_catalog.json`), or `null`. |
| `target_device_id` | string | Device the offer applies to (`device_id` in `product_catalog.json`), or `null`. |
| `discount_amount` | number | Discount value (monthly for plan offers, one-time for device/bundle, currency). |
| `eligible_account_statuses` | array | Account statuses the offer is valid for: `Active`, `Prospect`, `Delinquent`. |
| `valid_until` | string | Offer expiry timestamp (ISO 8601, UTC). |

### Data (5 promotions)

| promotion_id | title | type | target | discount | eligible statuses | valid_until |
|---|---|---|---|---|---|---|
| PROMO-0001 | Premium Upgrade Discount | PLAN_OFFER | PLAN-003 · Premium | $10/mo × 12 mo | Active, Prospect | 2026-12-03 |
| PROMO-0002 | Family Bundle Deal | PLAN_OFFER | PLAN-004 · Family | $15/mo (2+ lines) | Active | 2026-12-03 |
| PROMO-0003 | New Device Trade-In Bonus | DEVICE_PROMO | any flagship (with trade-in) | $200 off | Active, Prospect | 2026-12-03 |
| PROMO-0004 | Premium + Pixel Bundle | BUNDLE | PLAN-003 + DEV-1005 · Pixel 8 | $150 off | Active | 2026-12-03 |
| PROMO-0005 | Win-Back Unlimited Offer | PLAN_OFFER | PLAN-005 · Unlimited | Unlimited at $75/mo | Delinquent | 2026-12-03 |

Stacking rules (see B.11): max 2 offers per interaction; bundles don't stack with individual offers; win-back stacks with nothing. `eligible_account_statuses` gates eligibility — only `PROMO-0005` is valid for Delinquent accounts.

---

### A.5 Device Usage & Network Telemetry — `usage_telemetry.json` *(pending)*

**Purpose:** Drives personalized plan recommendations based on actual consumption.

This file is part of the platform schema (the basis for usage-vs-allowance comparisons such as the 60% downgrade trigger in B.1) but is **not yet populated** in the current data set. Schema reference:

### Record: UsageTelemetry

| Field | Type | Description |
|---|---|---|
| `line_id` | string | Reference to the customer/line (see `crm_records.json`). |
| `billing_cycle` | string | Cycle identifier (YYYY-MM). |
| `data_usage_gb` | array | 3-month historical array of GB consumed (oldest → newest). |
| `5g_data_pct` | number | Percentage of data consumed on 5G network (0–100). |
| `voice_minutes_used` | integer | Total voice minutes used in the cycle. |
| `international_calls_min` | number | International call minutes, if any. |
| `top_roaming_countries` | array | Countries where roaming was used, ranked by usage. |
| `current_device_battery_health_pct` | integer | Battery health percentage of the current device (0–100). |

### Example (format)

```json
{
  "line_id": "CUST-939042",
  "billing_cycle": "2026-08",
  "data_usage_gb": [18.4, 21.1, 24.9],
  "5g_data_pct": 82.0,
  "voice_minutes_used": 312,
  "international_calls_min": 45.0,
  "top_roaming_countries": ["Mexico", "Canada"],
  "current_device_battery_health_pct": 78
}
```

---

## Category B — Knowledge Base (Policy & Policy-Driven Processes)

Authored markdown documents that define the operational, legal, and financial rules the AI engine must honor when making recommendations. These are ingested as reference knowledge (not structured query data) and are grounded against Category A structured records before answering.

### B.1 Billing & Invoice Policy — `billing_invoice_policy.md.txt`

**Document ID:** TEL-BIL-001 · **v2.0** (Aug 2026) · **Department:** Finance Operations

**Purpose:** Governs monthly invoice generation, autopay behavior, account status definitions, and bill optimization guidance.

Key rules used by the engine:
- Invoices issued monthly with a 15-day due window.
- Autopay discounts $5/month on Premium and Unlimited plans; failed autopay attempts incur a $5 retry fee after 2 attempts.
- Balance younger than 30 days = Active; older than 30 days = Delinquent; disconnection at 45 days past due.
- Bill optimization triggers: usage < 60% of allowance for 2 consecutive cycles → recommend downgrade; multiple single lines → recommend Family plan; autopay off → encourage enrollment.

### B.2 Data Plan Terms & Conditions — `data_plan_terms_conditions.md.txt`

**Document ID:** TEL-PLN-003 · **v3.0** (Jun 2026) · **Department:** Legal & Compliance

**Purpose:** Governs data allowances, speeds, throttling, and add-ons across all postpaid plans.

Key rules used by the engine:
- Fair usage policy: personal, non-commercial use only; reselling and tethering beyond stated limits prohibited.
- Throttling: Essential → 128 kbps, Standard → 256 kbps after allowance; Unlimited deprioritized during congestion.
- Add-ons are one-time, cycle-valid, non-rollover purchases billed on the next invoice.
- Term changes require 30 days written notice.

### B.3 Device Protection Policy — `device_protection_policy.md.txt`

**Document ID:** TEL-DEV-003 · **v1.4** (Jun 2026) · **Department:** Device Services

**Purpose:** Defines device protection tiers, claim process, limits, and exclusions for accidental damage, loss, and theft.

Key rules used by the engine:
- Tiers: Screen Repair $5/mo ($29 deductible), Full Protection $12/mo ($99 deductible), Loss & Theft $16/mo ($199 deductible).
- Max 2 claims in 12 months per device; screen repair claims don't count toward the limit.
- Loss/theft requires a police report within 7 days.
- Coverage must be added within 30 days of purchase.

### B.4 Device Trade-In Process — `device_trade_in_process.md.txt`

**Document ID:** TEL-DEV-002 · **v2.1** (Jul 2026) · **Department:** Device Finance

**Purpose:** Defines device trade-in eligibility, valuation factors, process steps, and promotion stacking.

Key rules used by the engine:
- Device must be owned by account holder, power on and hold charge, account Active.
- Valuation factors: make/model, age, screen condition (cracks −30–50%), battery health (< 80% reduces value), activation lock, accessories.
- Trade-in credit applied instantly at point of sale, subject to facility inspection.
- Stacks with device promotions (e.g., PROMO-0003) but not with other device credits.

### B.5 Device Upgrade Guidelines — `device_upgrade_guidelines.md.txt`

**Document ID:** TEL-DEV-001 · **v1.6** (Aug 2026) · **Department:** Device Finance

**Purpose:** Defines device upgrade eligibility, installment financing terms, and trade-in valuation guidance.

Key rules used by the engine:
- Upgrade eligibility: device on account ≥ 12 months (or 50% of installment term), ≤ 2 unpaid installments, Active account.
- 0% APR when financed on Premium or Unlimited plans.
- Flagships < 2 years old trade in for 40–60% of original retail; broken screens reduce value 30–50%.
- Early payoff allowed any time; promotional credits may be forfeited.

### B.6 New Line Activation Process — `new_line_activation_process.md.txt`

**Document ID:** TEL-SRV-002 · **v1.7** (Jun 2026) · **Department:** Retail Operations

**Purpose:** Defines the process for activating new lines, including identity verification, SIM/eSIM provisioning, and first-bill expectations.

Key rules used by the engine:
- Pre-approval: two valid documents (ID + address proof), credit check for device financing, KYC consent.
- Physical SIM, eSIM (QR code), or remote provisioning.
- First bill includes prorated charges from activation date.
- 14-day return window starts at activation; activation fee waived for in-store activations on Premium+ plans.

### B.7 Payment & Eligibility Terms — `payment_eligibility_terms.md.txt`

**Document ID:** TEL-BIL-002 · **v1.9** (May 2026) · **Department:** Finance Operations

**Purpose:** Defines accepted payment methods, installment payment plan eligibility, and dispute resolution.

Key rules used by the engine:
- Payment methods: card, ACH, in-store cash/card, digital wallet.
- Installment plans: Active status + 12+ months tenure, $100 minimum split, max 3 installments; not for delinquent accounts or accounts in fraud review.
- Disputes must be filed within 60 days of invoice date; disputed amounts held during review.

### B.8 Plan Upgrade Policy — `plan_upgrade_policy.md.txt`

**Document ID:** TEL-PLN-002 · **v1.8** (Jul 2026) · **Department:** Consumer Plans

**Purpose:** Defines when and how customers may upgrade to higher-tier plans, eligibility, and promotional pricing behavior.

Key rules used by the engine:
- Eligibility: Active account, plan on account ≥ 30 days, no arrears on device installments, KYC within 12 months.
- Upgrades effective immediately with prorated charges; downgrades effective next billing cycle.
- Promotions stack only if explicitly stated; representative must disclose post-promotion price.
- Family plans change as a unit; per-line add-ons available instead.

### B.9 Port-In Policy — `port_in_policy.md.txt`

**Document ID:** TEL-SRV-001 · **v2.4** (Jul 2026) · **Department:** Carrier Services

**Purpose:** Governs number porting from another carrier, including eligibility, required info, timelines, and failure handling.

Key rules used by the engine:
- Number must be active and in good standing on the losing carrier; TPIN and account number required.
- Same-region ports: 2–4 hours; cross-region: 24–48 hours; corporate: 3–5 business days.
- Common failures: incorrect TPIN, name/address mismatch, suspended line, pending obligations.
- Zero-day trouble resolution via Port Desk callback queue.

### B.10 Postpaid Plan Guidelines — `postpaid_plan_guidelines.md.txt`

**Document ID:** TEL-PLN-001 · **v2.3** (Aug 2026) · **Department:** Consumer Plans

**Purpose:** Canonical plan catalog for postpaid plans — categories, allowances, overage behavior, and features.

Key rules used by the engine:
- Plan tiers: Essential $40 (5 GB), Standard $60 (20 GB), Premium $80 (50 GB), Family $120 (100 GB shared, 2+ lines), Unlimited $90.
- No hard caps above Essential/Standard — deprioritization instead of overage billing.
- Downgrades effective next billing cycle; upgrades immediate with proration.
- 14-day cooling-off period for new lines (see Returns & Refunds Policy).

### B.11 Promotions & Offer Eligibility Terms — `promotions_eligibility_terms.md.txt`

**Document ID:** TEL-PRO-001 · **v1.5** (Jul 2026) · **Department:** Marketing Operations

**Purpose:** Defines the offer catalog, offer matching, and stacking rules (complementary to the structured `promotions.json`).

Key rules used by the engine:
- Offer catalog: PROMO-0001 (Premium upgrade, $10/mo × 12), PROMO-0002 (Family bundle, $15/mo), PROMO-0003 (trade-in bonus, $200), PROMO-0004 (Premium + Pixel bundle, $150), PROMO-0005 (win-back Unlimited $75/mo).
- Max 2 offers per interaction; bundles don't stack with individual offers; win-back is exclusive to delinquent accounts.
- Stacking allowed: plan discount + device trade-in; not allowed: two plan discounts, win-back + any other.

### B.12 Returns & Refunds Policy — `returns_refunds_policy.md.txt`

**Document ID:** TEL-SRV-003 · **v3.1** (Aug 2026) · **Department:** Retail Operations

**Purpose:** Defines return windows, restocking fees, refund mechanics, and DoA handling.

Key rules used by the engine:
- Return windows: new device 14 days, eSIM/activation-only 14 days, open-box/refurbished 7 days.
- Refund to original payment method within 7–10 business days; plan days used deducted.
- $35 restocking fee for opened devices (waived for DoA/defective).
- Activation lock must be removed; device in original packaging with accessories.

---

## Category C — Operational Data (Sessions, Recommendations, Performance & Audits)

Operational logs produced by the platform at runtime. Unlike Categories A and B (ground truth / static knowledge), these are event-stream records generated by the Copilot application itself — used for performance monitoring, SLA verification, compliance auditing, and continuous model/UX improvement.

### C.7 Representative Session & Action Log — `rep_session_action.json`

**Purpose:** Tracks rep interactions within the Sidecar UI during a store transaction.

### Record: RepSession

| Field | Type | Description |
|---|---|---|
| `session_id` | string | Unique store-transaction session identifier. |
| `store_id` | string | Store where the session occurred. |
| `rep_id` | string | Representative identifier. |
| `session_start_utc` | string | Session start timestamp (ISO 8601, UTC). |
| `session_end_utc` | string | Session end timestamp (ISO 8601, UTC). |
| `aht_duration_seconds` | integer | Average Handling Time for the transaction. |
| `apps_switched_count` | integer | Number of application switches during the session. |
| `customer_lookup_duration_seconds` | integer | Time spent looking up the customer record. |

### Example

```json
{
  "session_id": "SES-88231",
  "store_id": "STORE-0101",
  "rep_id": "REP-5512",
  "session_start_utc": "2026-08-12T14:03:22Z",
  "session_end_utc": "2026-08-12T14:41:07Z",
  "aht_duration_seconds": 2265,
  "apps_switched_count": 4,
  "customer_lookup_duration_seconds": 48
}
```

---

### C.8 Copilot Recommendation Output — `copilot_recommendation.json`

**Purpose:** Captures what the AI recommended vs. what decision the rep actually made.

### Record: Recommendation

| Field | Type | Description |
|---|---|---|
| `recommendation_event_id` | string | Unique recommendation event identifier. |
| `session_id` | string | Reference to the originating session (`rep_session_action.json`). |
| `recommended_plan_id` | string | Plan the AI recommended (see `product_catalog.json`). |
| `confidence_score` | number | Model confidence for the recommendation (0–1). |
| `monthly_cost_delta` | number | Change in monthly cost versus current plan (currency). |
| `rep_decision` | string | `ACCEPTED`, `OVERRIDDEN`, or `REJECTED`. |
| `override_reason_code` | string | Reason code when the rep overrode/rejected the AI. |

### Example

```json
{
  "recommendation_event_id": "REC-001204",
  "session_id": "SES-88231",
  "recommended_plan_id": "PLAN-005",
  "confidence_score": 0.87,
  "monthly_cost_delta": 10.0,
  "rep_decision": "OVERRIDDEN",
  "override_reason_code": "CUSTOMER_PREFERS_CHEAPER"
}
```

---

### C.9 System Performance & Latency Telemetry — `system_telemetry.json`

**Purpose:** Monitors system responsiveness against the <1.5 s SLA.

### Record: SystemTelemetry

| Field | Type | Description |
|---|---|---|
| `trace_id` | string | Request trace identifier for end-to-end correlation. |
| `timestamp_utc` | string | Event timestamp (ISO 8601, UTC). |
| `api_fetch_latency_ms` | integer | Latency of structured API fetches (Category A data). |
| `rag_retrieval_latency_ms` | integer | Latency of RAG retrieval over Category B knowledge base. |
| `llm_inference_latency_ms` | integer | Latency of the LLM inference step. |
| `total_end_to_end_latency_ms` | integer | Total request latency (must be < 1500 ms per SLA). |
| `http_status_code` | integer | HTTP status returned for the request. |

### Example

```json
{
  "trace_id": "TRC-9f2c1a",
  "timestamp_utc": "2026-08-12T14:03:27Z",
  "api_fetch_latency_ms": 212,
  "rag_retrieval_latency_ms": 540,
  "llm_inference_latency_ms": 498,
  "total_end_to_end_latency_ms": 1250,
  "http_status_code": 200
}
```

---

### C.10 Compliance & Guardrail Audit Log — `guardrail_audit.json`

**Purpose:** Verifies that PII redaction and RAG grounding checks are working.

### Record: GuardrailAudit

| Field | Type | Description |
|---|---|---|
| `audit_event_id` | string | Unique audit event identifier. |
| `session_id` | string | Reference to the originating session. |
| `pii_redaction_status` | string | `PASSED` or `FAILED`. |
| `masked_fields` | array | Field names that were masked/redacted. |
| `cpni_consent_verified` | boolean | Whether CPNI consent was verified before using account data. |
| `rag_grounding_score` | number | Grounding/faithfulness score of the RAG response (0–1). |
| `price_verification_source` | string | Source confirming price accuracy, e.g. `CATALOG_API_VERIFIED`. |

### Example

```json
{
  "audit_event_id": "AUD-552018",
  "session_id": "SES-88231",
  "pii_redaction_status": "PASSED",
  "masked_fields": ["customer_id_hashed"],
  "cpni_consent_verified": true,
  "rag_grounding_score": 0.94,
  "price_verification_source": "CATALOG_API_VERIFIED"
}
```

---

### C.11 Representative Feedback & CSAT Log — `rep_feedback.json`

**Purpose:** Captures field sentiment and identifies poor AI responses.

### Record: RepFeedback

| Field | Type | Description |
|---|---|---|
| `feedback_id` | string | Unique feedback identifier. |
| `recommendation_event_id` | string | Reference to the recommendation being rated (`copilot_recommendation.json`). |
| `rep_id` | string | Representative providing the feedback. |
| `rating_thumbs` | string | `UP` or `DOWN`. |
| `feedback_category` | string | e.g. `WRONG_PROMO`, `SLOW_RESPONSE`, `EXCELLENT_PITCH`. |
| `free_text_comment` | string | Optional free-text comment. |

### Example

```json
{
  "feedback_id": "FB-77120",
  "recommendation_event_id": "REC-001204",
  "rep_id": "REP-5512",
  "rating_thumbs": "DOWN",
  "feedback_category": "WRONG_PROMO",
  "free_text_comment": "Recommended a promo the customer already used last month."
}
```

---

## Category Relationships

```
[Category A — structured ground truth (internal joins)]
crm_records.json ────────────── customer_id / billing_account_id ──▶ billing_records.json
crm_records.json ────────────── current_plan_id ──────────────────▶ product_catalog.json (plans)
crm_records.json ────────────── eligible_promotion_ids ───────────▶ promotions.json
billing_records.json ────────── device_id ────────────────────────▶ product_catalog.json (devices)
promotions.json ─────────────── target_plan_id ───────────────────▶ product_catalog.json
(usage_telemetry.json is pending — will join crm_records.json/plan products for usage-driven matching)

[Category A — structured ground truth]        [Category B — policy knowledge]
crm_records.json ───────────────────────────────────────────▶ plan_upgrade_policy.md.txt        (account status, tenure gates)
crm_records.json / billing_records.json ───────────────────▶ billing_invoice_policy.md.txt     (invoice & autopay behavior)
product_catalog.json ──────────────────────────────────────▶ postpaid_plan_guidelines.md.txt   (plan features & pricing)
product_catalog.json ──────────────────────────────────────▶ data_plan_terms_conditions.md.txt (allowance & throttling)
product_catalog.json ──────────────────────────────────────▶ device_upgrade_guidelines.md.txt  (installment financing)
product_catalog.json ──────────────────────────────────────▶ device_trade_in_process.md.txt    (device lineup)
promotions.json ───────────────────────────────────────────▶ promotions_eligibility_terms.md.txt (offer & stacking rules)

[Category C — operational logs]
rep_session_action.json ──session_id──▶ copilot_recommendation.json ──recommendation_event_id──▶ rep_feedback.json
copilot_recommendation.json ──recommended_plan_id──▶ product_catalog.json / promotions.json
system_telemetry.json ──latency breakdown──▶ AI engine SLA (< 1.5 s) monitoring
guardrail_audit.json ──session_id──▶ rep_session_action.json (compliance correlation)
```

The AI engine joins Category A structured records with Category B policy documents to answer representative and customer queries (e.g. best plan, upgrade eligibility, promo stacking) in a single conversation. Category C logs continuously measure that experience — rep adoption (C.7), AI-rep agreement (C.8), SLA adherence (C.9), safety/compliance (C.10), and field sentiment (C.11).