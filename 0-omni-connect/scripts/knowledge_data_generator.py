"""
Knowledge Base Data Generator
Generates the Category B knowledge base documents (policies, processes,
eligibility terms, terms & conditions) for the omni-connect telecom retail domain.

Each curated document carries its canonical metadata (title, Document ID,
version, last-updated date, department) and body text inline — mirroring the
Category B index in docs/data.md. No file is read at runtime.

Output mirrors docs/data.md Category B and is written to:
    data/knowledge_base/<policy_name>.md.txt
        - billing_invoice_policy.md.txt
        - data_plan_terms_conditions.md.txt
        - device_protection_policy.md.txt
        - device_trade_in_process.md.txt
        - device_upgrade_guidelines.md.txt
        - new_line_activation_process.md.txt
        - payment_eligibility_terms.md.txt
        - plan_upgrade_policy.md.txt
        - port_in_policy.md.txt
        - postpaid_plan_guidelines.md.txt
        - promotions_eligibility_terms.md.txt
        - returns_refunds_policy.md.txt

Std-lib only. Run:  python scripts/knowledge_data_generator.py
"""

from dataclasses import dataclass
from pathlib import Path

# ==============================================================================
# 0. OUTPUT PATH
# ==============================================================================

ROOT = Path(__file__).resolve().parent.parent
OUTPUT_DIR = ROOT / "data" / "knowledge_base"

# ==============================================================================
# 1. KNOWLEDGE DOCUMENT SCHEMA
# ==============================================================================


@dataclass
class KnowledgeMetadata:
    title: str
    doc_id: str
    version: str
    last_updated: str
    department: str


@dataclass
class KnowledgeDocument:
    metadata: KnowledgeMetadata
    body: str

    @property
    def rendered(self) -> str:
        m = self.metadata
        header = (
            f"# {m.title}\n\n"
            f"## Document Information\n"
            f"- **Document ID:** {m.doc_id}\n"
            f"- **Version:** {m.version}\n"
            f"- **Last Updated:** {m.last_updated}\n"
            f"- **Department:** {m.department}\n\n"
            f"---\n\n"
        )
        return header + self.body.strip() + "\n"


# ==============================================================================
# 2. CURATED KNOWLEDGE DOCUMENTS (canonical metadata + curated body text)
#     Keyed by the docs/data.md filenames (B.1–B.12).
# ==============================================================================

KB = {
    "postpaid_plan_guidelines.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Postpaid Plan Guidelines",
            doc_id="TEL-PLN-001",
            version="2.3",
            last_updated="August 2026",
            department="Consumer Plans",
        ),
        "body": """## Overview

Postpaid plans cover monthly rate plans for individual and family lines. This document covers plan categories, allowances, overage behavior, and features per plan.

## Plan Categories

### 1. Essential
- Entry-level plan for light users
- 5 GB high-speed data, 500 talk minutes, 500 SMS
- Monthly price: $40
- No hotspot, no international calling

### 2. Standard
- Mid-tier plan for everyday users
- 20 GB high-speed data, 1,000 talk minutes, 1,000 SMS
- Monthly price: $60
- Mobile hotspot included

### 3. Premium
- High-tier plan for heavy users
- 50 GB high-speed data, unlimited talk & text
- Monthly price: $80
- Hotspot and international calling included

### 4. Family
- Multi-line plan for households (2+ lines)
- 100 GB shared data, unlimited talk & text
- Monthly price: $120
- Requires at least two lines on the account

### 5. Unlimited
- Truly unlimited high-speed data, talk, and text
- Monthly price: $90
- Hotspot and international calling included

---

## Data Allowance Behavior

| Plan | High-Speed Data | After Allowance |
|------|-----------------|-----------------|
| Essential | 5 GB | Throttled to 128 kbps |
| Standard | 20 GB | Throttled to 256 kbps |
| Premium | 50 GB | Throttled to 512 kbps |
| Family | 100 GB shared | Throttled to 256 kbps per line |
| Unlimited | Unlimited | Priority below Essentials users during congestion |

**Overage:** No hard data caps on Premium, Family, or Unlimited. Essential and Standard deprioritize instead of overage billing.

---

## Plan Features Matrix

| Feature | Essential | Standard | Premium | Family | Unlimited |
|---------|-----------|----------|---------|--------|-----------|
| Talk (min) | 500 | 1,000 | Unlimited | Unlimited | Unlimited |
| Text | 500 | 1,000 | Unlimited | Unlimited | Unlimited |
| Hotspot | No | Yes | Yes | Yes | Yes |
| Intl. Calling | No | No | Yes | Yes | Yes |

---

## Change Policies

- **Downgrade:** Effective from next billing cycle.
- **Upgrade:** Effective immediately; prorated charges apply.
- **Mid-cycle changes:** Full plan price billed on change date.
- 14-day cooling-off period for new lines (see Returns & Refunds Policy).

---

## FAQs

**Q: Can I share my data with other lines?**
A: On Family plans data is automatically shared. Other plans can add data sharing for a fee.

**Q: What happens when I travel abroad?**
A: Premium, Family, and Unlimited include international calling. Essential and Standard require a travel add-on.

**Q: Can I switch plans at any time?**
A: Yes, subject to plan activation dates and proration rules above.

---

**Contact Plan Team:**
- Plan Helpline: 1800-XXX-PLAN
- Email: plans@omniconnect.com
- Visit nearest retail store

*Plans and pricing subject to change. Terms and conditions apply.*""",
    },
    "plan_upgrade_policy.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Plan Upgrade Policy",
            doc_id="TEL-PLN-002",
            version="1.8",
            last_updated="July 2026",
            department="Consumer Plans",
        ),
        "body": """## Overview

This policy defines when and how a customer may move to a higher-tier plan, how eligibility is determined, and how promotional pricing applies.

## Eligibility Criteria

A customer is eligible for an immediate plan upgrade when:

- Account is in **Active** status (no outstanding balance older than 30 days)
- Current plan has been on the account for at least **30 days**
- No unpaid device installment balance in arrears
- Identity verification is current (KYC within last 12 months)

**Exclusions:**
- Accounts flagged for fraud or abuse
- Delinquent accounts (see Account Status below)

---

## Account Status Rules

| Account Status | Upgrade Allowed | Notes |
|----------------|-----------------|-------|
| Active | Yes | Immediate processing |
| Prospect | Yes | Requires new plan activation first |
| Delinquent | No | Must settle balance, then upgrade |

---

## Upgrade Process

1. Representative confirms eligibility (see criteria above).
2. Recommend target plan based on usage analysis (60-day average data/talk).
3. Customer consents to the new plan price and effective date.
4. Apply any applicable promotion (e.g., PROMO-0001 Premium Upgrade Discount).
5. System applies the change; customer receives confirmation SMS.

---

## Proration Rules

- Upgrades take effect **immediately** on the change date.
- The customer pays the difference between old and new monthly price for the remainder of the cycle.
- Device installment schedules are unaffected by plan changes.

---

## Promotional Pricing

- Promotions stack only if explicitly stated (e.g., bundle offers).
- Discount duration noted at offer time; plan returns to standard price after expiry.
- Representative must inform customer of post-promotion price.

---

## FAQs

**Q: How soon can I upgrade after joining?**
A: Typically 30 days after activation, or earlier during promotional windows.

**Q: Do I lose my number when I upgrade?**
A: No. Number portability is independent of plan changes.

**Q: Can I upgrade a single line on a Family plan?**
A: No. Family plans change as a unit; per-line add-ons are available instead.

---

**Contact Plan Team:**
- Plan Helpline: 1800-XXX-PLAN
- Email: plans@omniconnect.com

*Terms and conditions apply.*""",
    },
    "data_plan_terms_conditions.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Data Plan Terms & Conditions",
            doc_id="TEL-PLN-003",
            version="3.0",
            last_updated="June 2026",
            department="Legal & Compliance",
        ),
        "body": """## Overview

These terms govern all data allowances, speeds, and add-ons across omni-connect postpaid plans.

## Fair Usage Policy

- Data allowances are for personal, non-commercial use.
- Unlimited plans are subject to network management during congestion.
- Automated downloading, tethering beyond stated limits, and reselling are prohibited.

---

## Speed & Throttling

| Condition | Result |
|-----------|--------|
| Allowance exhausted (Essential) | Throttled to 128 kbps |
| Allowance exhausted (Standard) | Throttled to 256 kbps |
| Congestion (Unlimited) | Prioritized below postpaid Essentials |
| International roaming data | Speed reduced to 3G/4G as per destination |

---

## Data Add-ons

- Add-ons are one-time purchases valid for the current billing cycle.
- Unused add-on data does not roll over.
- Add-on charges billed in the next invoice.

---

## Bill Optimization Guidance

Customers seeking to reduce monthly bills should consider:

1. Moving to a lower tier if 60-day average usage < 60% of allowance.
2. Enabling autopay for a recurring discount.
3. Consolidating multiple lines onto a Family plan.
4. Using Wi-Fi calling to reduce talk-minute usage.

---

## Compliance Notes

- All terms are part of the subscriber agreement.
- Changes to terms require 30 days written notice.
- Customers may terminate without penalty upon material term changes.

---

**Contact Legal:**
- Email: legal@omniconnect.com

*Terms subject to regulatory change.*""",
    },
    "device_upgrade_guidelines.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Device Upgrade Guidelines",
            doc_id="TEL-DEV-001",
            version="1.6",
            last_updated="August 2026",
            department="Device Finance",
        ),
        "body": """## Overview

Device upgrades allow customers to move to a newer handset, typically with trade-in, installment, or promotional pricing. This document covers eligibility, timing, and financing.

## Upgrade Eligibility

- Device must be on the account for at least **12 months** (or 50% of installment term).
- No more than **2 unpaid installments** on the current device.
- Account status: Active (see Plan Upgrade Policy for status rules).

### Early Upgrade Options

| Situation | Option |
|-----------|--------|
| 12+ months on device | Standard upgrade |
| Device damaged | Device protection claim, then upgrade |
| Device lost/stolen | Protect plan payout, then upgrade |

---

## Installment Financing

| Device | Retail Price | Term | Est. /month |
|--------|--------------|------|-------------|
| iPhone 15 | $799 | 24 mo | $33.30 |
| iPhone 15 Pro | $1,099 | 24 mo | $45.80 |
| Galaxy S24 | $799 | 24 mo | $33.30 |
| Galaxy S24 Ultra | $1,199 | 24 mo | $50.00 |
| Pixel 8 | $699 | 24 mo | $29.10 |

- 0% APR when device is financed on a Premium or Unlimited plan.
- Down payment required on non-flagship financing.

---

## Trade-In Value

See `device_trade_in_process.md.txt` for the full trade-in procedure. Typical valuation:

- Flagship phones < 2 years old: 40–60% of original retail price.
- Mid-range phones < 3 years old: 25–40%.
- Broken screens or cracked backs reduce value by 30–50%.

---

## Upgrade Process

1. Confirm account status and installment eligibility.
2. Recommend target device from usage and preference.
3. Apply trade-in credit (see Trade-In Process).
4. Process new installment agreement.
5. Old device shipped to trade-in facility within 7 days.

---

## FAQs

**Q: Can I pay off my device early?**
A: Yes, full balance can be paid at any time; promotional credits may be forfeited.

**Q: Can I transfer my device to another line?**
A: Yes, if both lines are on the same account.

**Q: Do I keep my old number on upgrade?**
A: Yes, number and plan remain unchanged.

---

**Contact Device Team:**
- Device Helpline: 1800-XXX-DEV
- Email: devices@omniconnect.com

*Terms and conditions apply.*""",
    },
    "device_trade_in_process.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Device Trade-In Process",
            doc_id="TEL-DEV-002",
            version="2.1",
            last_updated="July 2026",
            department="Device Finance",
        ),
        "body": """## Overview

The trade-in program credits customers for their current device toward a new purchase. This process document details assessment, valuation, and credit application.

## Eligibility

- Original device must be owned by the account holder.
- Device must power on and hold a charge for valuation.
- Account must be Active (see Plan Upgrade Policy).

---

## Valuation Factors

| Factor | Influence |
|--------|-----------|
| Make & model | Base value table |
| Age | Depreciation over time |
| Screen condition | Cracks reduce value 30–50% |
| Battery health | < 80% capacity reduces value |
| Activation lock | Must be removed before valuation |
| Accessories | Charger/box increase value by $10–20 |

---

## Trade-In Steps

1. Customer selects new device and opts into trade-in.
2. Representative runs device diagnostics (screen, battery, IMEI).
3. System quotes trade-in value instantly.
4. Customer accepts quote.
5. Trade-in credit applied immediately to new device.
6. Old device collected in-store or shipped via prepaid label (7-day window).
7. Credit finalized after facility inspection (if mismatch, customer billed difference).

---

## Promotion Stacking

- Trade-in credit can stack with device promotions such as PROMO-0003 (New Device Trade-In Bonus, $200 off).
- Trade-in credit cannot be combined with other device credits.

---

## FAQs

**Q: Will I get a box to ship my phone?**
A: Yes, a prepaid shipping kit is provided in-store or by mail.

**Q: What if my phone has a cracked screen?**
A: It is still eligible; value is reduced per the valuation table.

**Q: When is my credit applied?**
A: Instantly at point of sale, subject to inspection confirmation.

---

**Contact Device Team:**
- Email: devices@omniconnect.com

*Valuations subject to physical inspection.*""",
    },
    "device_protection_policy.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Device Protection Policy",
            doc_id="TEL-DEV-003",
            version="1.4",
            last_updated="June 2026",
            department="Device Services",
        ),
        "body": """## Overview

Device protection covers accidental damage, loss, and theft. This policy describes plan tiers, claims, and deductibles.

## Coverage Tiers

### Tier 1 — Screen Repair
- Front screen replacement
- Monthly: $5
- Deductible: $29 per claim

### Tier 2 — Full Protection
- Accidental damage, liquid damage, mechanical breakdown
- Monthly: $12
- Deductible: $99 per claim

### Tier 3 — Loss & Theft
- Includes Full Protection coverage plus loss/theft replacement
- Monthly: $16
- Deductible: $199 per claim

---

## Claim Process

1. Customer reports incident via app or store.
2. Representative verifies coverage and account status.
3. Deductible quoted and collected.
4. Replacement/repair initiated within 24 hours.
5. Replacement ships in 1–3 business days.

---

## Claim Limits

- Maximum of **2 claims in 12 months** per device.
- Screen repair claims do not count against Full Protection limits.
- Loss/theft requires police report (within 7 days).

---

## Exclusions

- Cosmetic wear and tear.
- Devices purchased more than 30 days before coverage start.
- Pre-existing damage at coverage purchase.
- Removal of activation lock cases.

---

## FAQs

**Q: Can I add protection after buying my phone?**
A: Yes, within 30 days of purchase.

**Q: What if my phone is lost while traveling?**
A: Loss & Theft tier covers worldwide; a police report is required.

**Q: How many claims can I file?**
A: Two per rolling 12 months.

---

**Contact Device Services:**
- Email: protection@omniconnect.com

*Deductibles and limits subject to change.*""",
    },
    "billing_invoice_policy.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Billing & Invoice Policy",
            doc_id="TEL-BIL-001",
            version="2.0",
            last_updated="August 2026",
            department="Finance Operations",
        ),
        "body": """## Overview

This policy governs how monthly invoices are generated, when autopay applies, and how balances are treated.

## Invoice Cycle

- Invoices are issued monthly with a **15-day due window**.
- Invoice includes plan charge, taxes & fees, add-ons, and device installments.
- Invoices are available digitally (app/portal) and by email.

---

## Charges on Invoice

| Line Item | Description |
|-----------|-------------|
| Monthly Rate Plan | Base plan price (e.g., Standard $60) |
| Regulatory Taxes & Fees | Taxes and surcharges (~10%) |
| Data / Talk Add-ons | One-time purchases |
| Device Installment | Monthly device payment |
| Service Fees | Late fees, chargebacks |

---

## Autopay

- Autopay enabled by default (75% of accounts).
- Autopay discounts $5/month on Premium and Unlimited plans.
- Payment method: primary card, debit card, or ACH.
- Failed autopay attempts incur a $5 retry fee after 2 attempts.

---

## Balance & Status

| Account Status | Definition |
|----------------|------------|
| Active | No balance older than 30 days |
| Delinquent | Balance outstanding > 30 days |
| Prospect | No active billing account yet |

- Current balance carries to next invoice if unpaid.
- Disconnection at 45 days past due (see Service & Disconnect Policy).

---

## Bill Optimization

Customers eligible for optimization guidance:

- Usage < 60% of allowance for 2 consecutive cycles → recommend downgrade.
- Multiple single lines on an account → recommend Family plan.
- Autopay not active → encourage enrollment for discount.

---

## FAQs

**Q: Why is my total higher than my plan price?**
A: Total includes taxes, fees, add-ons, and device installments.

**Q: Can I change my billing date?**
A: Billing dates are fixed by account creation; changes require manager approval.

**Q: How do I get a copy of an old invoice?**
A: Available digitally for the last 12 months.

---

**Contact Billing:**
- Billing Helpline: 1800-XXX-BILL
- Email: billing@omniconnect.com

*Billing terms apply.*""",
    },
    "payment_eligibility_terms.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Payment & Eligibility Terms",
            doc_id="TEL-BIL-002",
            version="1.9",
            last_updated="May 2026",
            department="Finance Operations",
        ),
        "body": """## Overview

These terms define accepted payment methods, payment plan eligibility, and dispute resolution for billed amounts.

## Accepted Payment Methods

- Credit / Debit card
- ACH (bank account)
- In-store cash and card payments
- Digital wallet (app)

---

## Payment Method Requirements

| Method | Requirement |
|--------|-------------|
| Credit/Debit | Non-expired, valid billing address |
| ACH | Verified bank account, 3–5 business day settlement |
| Digital wallet | Linked funding source on file |

---

## Payment Plans (Installments)

Eligible customers (Active status, 12+ months tenure) may split an invoice into installments:

- Minimum split: $100 outstanding.
- Maximum term: 3 installments.
- Late payment fees apply per installment.

**Not eligible:**
- Delinquent accounts with prior defaults.
- Accounts in active fraud review.

---

## Disputes

- Disputes must be filed within **60 days** of invoice date.
- Credit is provided for validated billing errors.
- Disputed amounts are held during review (no late fee accrues).

---

## FAQs

**Q: Can I pay from a bank account of another person?**
A: No, the funding bank account must belong to the account holder.

**Q: How quickly do payments post?**
A: Card payments post instantly; ACH takes 3–5 business days.

**Q: Is there a fee to pay by card?**
A: No surcharge for card payment in region; reimbursement applies for ACH defaults.

---

**Contact Billing:**
- Email: billing@omniconnect.com

*Payment terms apply.*""",
    },
    "port_in_policy.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Port-In Policy",
            doc_id="TEL-SRV-001",
            version="2.4",
            last_updated="July 2026",
            department="Carrier Services",
        ),
        "body": """## Overview

Number porting brings an existing phone number from another carrier to omni-connect. This policy covers eligibility, timelines, and common failure reasons.

## Port-In Eligibility

- The number must be **active** on the losing carrier.
- Account on the losing carrier must be in good standing.
- A valid **transfer PIN (TPIN)** and account number are required.
- The number must not be mid-port from a recent move.

---

## Required Information

- Customer's phone number
- Losing carrier account number
- Transfer PIN (TPIN)
- Account holder verification (name, address)

---

## Porting Timelines

| Scenario | Timeline |
|----------|----------|
| Same region, all details correct | 2–4 hours |
| Cross-region / complex cases | 24–48 hours |
| Corporate lines | 3–5 business days |

---

## Common Failure Reasons

- Incorrect TPIN or account number.
- Name/address mismatch with losing carrier.
- Number on a suspended or terminated line.
- Pending obligations (e.g., device installment) on losing carrier.

---

## Zero-Day Trouble Resolution

- For issues at activation, the **Zero-Day Experience** callback queue applies.
- Store representatives initiate a trouble ticket before the customer leaves:
  1. Verify port details.
  2. Escalate via the internal Port Desk chat.
  3. Hand off to the Port Desk with a case number.

---

## FAQs

**Q: Can I cancel a port before it completes?**
A: Yes, before the port completes; after activation the number must be ported back.

**Q: Can I keep both plans active during the port?**
A: No, the port transfers service; overlapping charges apply only if requested.

**Q: What if my port fails?**
A: The store resolves with the Port Desk or re-attempts with corrected details.

---

**Contact Carrier Services:**
- Port Desk: 1800-XXX-PORT
- Email: port@omniconnect.com

*Ports subject to carrier schedules.*""",
    },
    "new_line_activation_process.md.txt": {
        "metadata": KnowledgeMetadata(
            title="New Line Activation Process",
            doc_id="TEL-SRV-002",
            version="1.7",
            last_updated="June 2026",
            department="Retail Operations",
        ),
        "body": """## Overview

Process for activating a new line, including identity verification, SIM/eSIM setup, and first-bill expectations.

## Pre-Approval Steps

1. Verify identity with two valid documents (ID + address proof).
2. Run credit check if installing a device.
3. Confirm customer consent for KYC data storage.
4. Select plan and device (or bring-your-own-device).

---

## Identity Verification

| Requirement | Accepted Document |
|-------------|-------------------|
| Identity | Passport, driver license, national ID |
| Address | Utility bill, bank statement (90 days) |
| In-store biometric | Optional for high-value activations |

---

## SIM / eSIM Provisioning

- **Physical SIM:** eSIM-ready stores provide instant physical SIM.
- **eSIM:** QR code issued, activation within minutes.
- **Remote provisioning (Phase 4+):** Service callbacks activate eSIM without store visit.

---

## Activation Flow

1. System creates the account and assigns billing cycle.
2. Customer receives welcome SMS.
3. First bill includes prorated charges from activation date.
4. 14-day return window starts at activation.

---

## First Bill Expectations

- Prorated plan charge for days remaining in cycle.
- Taxes and fees.
- Activation fee waived for in-store activations on Premium+ plans.

---

## FAQs

**Q: How long does activation take?**
A: 5–10 minutes with correct documentation.

**Q: Can I activate my own device?**
A: Yes (BYOD); only plan and SIM required.

**Q: What if I lose my SIM?**
A: Replacement SIM issued in 10 minutes; eSIM re-issued via app.

---

**Contact Retail Operations:**
- Retail Helpline: 1800-XXX-RTL

*Activation terms apply.*""",
    },
    "returns_refunds_policy.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Returns & Refunds Policy",
            doc_id="TEL-SRV-003",
            version="3.1",
            last_updated="August 2026",
            department="Retail Operations",
        ),
        "body": """## Overview

Customers may return devices and cancel lines within a cooling-off period. This policy defines windows, restocking fees, and refund mechanics.

## Return Windows

| Scenario | Window |
|----------|--------|
| New device (in-store or online) | 14 days |
| eSIM / activation only (no device) | 14 days |
| Open-box / refurbished device | 7 days |

---

## Refund Mechanics

- Full refund to original payment method within **7–10 business days**.
- Activation fee is refunded when the full line is canceled.
- Plan charges for days used are deducted from the refund.

---

## Device Return Conditions

- Device must be returned with original packaging and accessories.
- Cosmetic condition: no cracks or liquid damage.
- Activation lock must be removed.
- Restocking fee **$35** applies for opened devices (waived for DOA/defective).

---

## DoA (Dead on Arrival)

- Reported within 14 days.
- Replacement shipped next business day.
- No restocking fee.

---

## Process

1. Representative verifies purchase date and return window.
2. Device diagnostics run in-store.
3. Refund issued per policy above.
4. Line canceled or replaced; plan charges adjusted.

---

## FAQs

**Q: Can I return an activated device?**
A: Yes, within the 14-day window, subject to conditions above.

**Q: Who pays return shipping?**
A: omni-connect pays for factory defects and DoA; customer pays for change-of-mind returns.

**Q: When will I see my refund?**
A: Within 7–10 business days of device receipt.

---

**Contact Retail Operations:**
- Email: returns@omniconnect.com

*Returns policy applies.*""",
    },
    "promotions_eligibility_terms.md.txt": {
        "metadata": KnowledgeMetadata(
            title="Promotions & Offer Eligibility Terms",
            doc_id="TEL-PRO-001",
            version="1.5",
            last_updated="July 2026",
            department="Marketing Operations",
        ),
        "body": """## Overview

Promotions and offers provide discounts on plans, devices, and bundles. This document defines how offers are matched and what makes a customer eligible.

## Offer Catalog

| Offer ID | Name | Type | Benefit |
|----------|------|------|---------|
| PROMO-0001 | Premium Upgrade Discount | Plan offer | $10/mo off Premium for 12 months |
| PROMO-0002 | Family Bundle Deal | Plan offer | $15/mo off Family with 2+ lines |
| PROMO-0003 | New Device Trade-In Bonus | Device promo | $200 off flagship with trade-in |
| PROMO-0004 | Premium + Pixel Bundle | Bundle | Premium + Pixel 8, $150 off |
| PROMO-0005 | Win-Back Unlimited Offer | Plan offer | Unlimited at $75/mo |

---

## Eligibility by Account Status

| Account Status | Eligible Offers |
|----------------|-----------------|
| Active | PROMO-0001, PROMO-0002, PROMO-0003, PROMO-0004 |
| Prospect | PROMO-0001, PROMO-0003 |
| Delinquent | PROMO-0005 (win-back) |

---

## Offer Matching Rules

- Representative matches offers based on account status and intent.
- A maximum of **2 offers** offered per customer interaction.
- Bundles (PROMO-0004) cannot stack with individual plan or device offers.
- Win-back offers (PROMO-0005) are exclusive to delinquent accounts.

---

## Stacking

| Combination | Allowed |
|-------------|---------|
| Plan discount + device trade-in | Yes |
| Plan discount + bundle | No |
| Two plan discounts | No |
| Win-back + any other offer | No |

---

## Offer Duration

- Plan discounts: 12 months unless stated.
- Device credits: instant at purchase.
- Promotions expire at the `valid_until` date of the offer.

---

## FAQs

**Q: Can I get two promotions on one order?**
A: Yes, a plan offer plus a device promo (subject to stacking rules).

**Q: What happens after a discount expires?**
A: The plan returns to its standard price; you are notified in advance.

**Q: Why was I not offered a promotion?**
A: Eligibility is based on account status and current promotions; see table above.

---

**Contact Marketing:**
- Email: offers@omniconnect.com

*Offers subject to availability. Terms and conditions apply.*""",
    },
}


# ==============================================================================
# 3. EXECUTION & OUTPUT TO data/knowledge_base
# ==============================================================================

def main() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    print(f"Successfully generated knowledge base under '{OUTPUT_DIR.resolve()}':")
    for filename, spec in KB.items():
        path = OUTPUT_DIR / filename
        rendered = KnowledgeDocument(metadata=spec["metadata"], body=spec["body"]).rendered
        path.write_text(rendered, encoding="utf-8")
        print(f" - {path}")


if __name__ == "__main__":
    main()