# Omni-Connect Retail Copilot Portal — UI Reference

This document describes the functionality of the Streamlit portal in
`app/portal.py`. The portal is the representative-facing workspace where the
copilot surfaces customer context, prompts offers, and captures the final order
decision. It is a scaffold: business-data lookups are live, while the agent,
RAG, telemetry, and POS wiring are placeholders ready to be connected.

## Run

```bash
streamlit run app/portal.py
```

- Loads at `http://localhost:8501` by default (`--server.port` to override).
- Imports the business-data services from `src/services/` at runtime
  (`sys.path` wired to the repo root, mirroring the bootcamp convention).
- Requires the generated datasets under `data/business_data/` (produced by
  `scripts/business_data_generator.py`) and the knowledge base under
  `data/knowledge_base/` (produced by `scripts/knowledge_data_generator.py`).

## Layout

Fixed footer plus three side-by-side containers (`st.columns(3)`):

| Column | Container | Purpose |
|---|---|---|
| 1 | Customer | Selected customer's profile, line, usage, and billing context |
| 2 | Agent | Real-time copilot chat, promo eligibility cards, RAG citations, prompt shortcuts |
| 3 | Decision | Next-best action, financial delta, action/override capture, order submission |

## Customer Container

Interactive selector (`customer_selectbox`) over `customer_id`s loaded from
`crm_records.json`. All downstream panels re-render for the selection.

### Profile Header

| Field | Source | Notes |
|---|---|---|
| Customer ID | `crm_records.json` `customer_id` | Full ID shown (no masking) |
| Tenure | `created_at` | Computed `format_tenure()` → "4 Years" / "9 Months" |
| Account Status | `account_status` | `Active` / `Delinquent` / `Prospect` |
| Credit Tier | — | `—`, field not in schema yet |

### Line Details

| Field | Source | Notes |
|---|---|---|
| Line ID | `contact.phone` | Treated as the line identifier (MDN) |
| Current Rate Plan | `product_catalog.json` via `current_plan_id` | Live lookup |
| Contract End Date | — | `—`, not in schema yet |
| Device Model | `product_catalog.json` via billing `device_id` | Live lookup |
| Remaining Financing Balance | — | `—`, not in schema yet |

### Usage Telemetry

`3-Month Avg Data Usage (GB)`, `5G Usage %`, roaming and international call
flags. All `—` with a `usage_telemetry.json pending` note — that data source
is documented as pending in `docs/data.md` (A.5).

### Billing Context

| Field | Source | Notes |
|---|---|---|
| Monthly Bill Average | Last 6 invoices from `billing_records.json` | Mean `total_amount`, formatted `$44.00` |
| Auto-Pay Status | `autopay_enabled` | `Enabled` / `Disabled` |
| Payment Reliability | Invoice `status` history | `Reliable` (0 overdue) / `Occasionally Late` (≤ 2) / `At Risk` (> 2) |

## Agent Container

Houses the real-time AI assistant (UI scaffold — reasoning stream is not wired yet).

### Conversational Chat

- `st.session_state["copilot_messages"]` persists the `user`/`assistant`
  turn history across reruns; seeded with a welcome message.
- `st.chat_message` renders both roles; `st.chat_input` appends the user's
  query and returns a scaffold assistant acknowledgement referencing the
  selected customer, plan, and device.
- Streams no real rationale yet — placeholder until RAG + recommendation
  agents are connected.

### Promotional Eligibility Cards

Live: filters `promotions.json` to the customer's `eligible_promotion_ids`
and renders one bordered card per offer — title, description, promo type,
discount amount, and `valid_until`. Empty state caption when no offers match.

### RAG Knowledge Snippets

Expandable source citations ("document ID · title") via `load_kb_snippet()`,
which reads a policy document from `data/knowledge_base/` and extracts its
Document ID, version, and Overview excerpt. Base sources:
`promotions_eligibility_terms.md.txt`, `plan_upgrade_policy.md.txt`,
`billing_invoice_policy.md.txt`; `device_trade_in_process.md.txt` is added
when PROMO-0003 is eligible. Competitor battlecards are noted as not authored.

### Quick Prompt Shortcuts

`PROMPT_SHORTCUTS` buttons (`Compare with competitor X`, `Check trade-in rules`,
`Explain price change`); clicking injects a user turn and a placeholder
assistant response into the chat stream.

## Decision Container

Captures the representative's action, logs human-in-the-loop overrides, and
submits to a mock POS/Billing pipeline.

### Recommended Next-Best Action

`recommend_next_best_action()` — scaffold rule (standing in for the
recommendation agent), keyed on account status and a plan tier ladder
(`Essential → Standard → Premium → Unlimited`):

| Status | Recommendation |
|---|---|
| Delinquent | Settle balance, then re-qualify for win-back (PROMO-0005) |
| Prospect | Activate new line on the next tier + device |
| Active | Upgrade to the next tier + device (trade-in) — or "Keep current plan" on top tier / Family |

### Financial Delta

Monthly price change vs. the current plan, formatted `+$20.00/mo`,
`-$5.00/mo`, or `$0.00/mo`; "balance due" for delinquent accounts. Caption
notes the value is illustrative until trade-in credit is modeled.

### Action State & Override

- Horizontal `st.radio`: `Accept` / `Override` / `Reject`.
- `Override` reveals a required `st.selectbox` (`Price Resistance`, `Customer
  Prefers Legacy Plan`, `Device Not Available`, `Other`). Submission is
  disabled until a reason is chosen.

### System Telemetry Summary

`Latency: 1.2s | Grounding Score: 0.96` — static mock values, captioned as
pending observability instrumentation.

### Submit Order to POS

Primary button appends an order record to `st.session_state["pos_orders"]`:
`submitted_at` (UTC ISO), `customer_id`, `action`, `action_state`,
`override_reason`, `latency_ms`, `grounding_score`. Confirmation via
`st.success`; the last 5 submissions render in a "Recently Submitted Orders"
expander. This is the mock POS/Billing pipeline — wire a real endpoint later.

## Session State Summary

| Key | Type | Purpose |
|---|---|---|
| `copilot_messages` | list | Chat history (`role`/`content`) for the Agent chat |
| `pos_orders` | list | Submitted order log for the Decision container |
| `customer_selectbox` | str | Current customer selection |
| `action_state` / `override_reason` | str / str \| None | Decision widget state |

## Mock / Placeholder Inventory

| Feature | Status |
|---|---|
| Business-data lookups (CRM, billing, catalog, promotions) | Live via `src/services/` |
| Chat UI, shortcuts, chat_input | Live UI; no model/reasoning wired |
| RAG citations | Snippet extraction from `data/knowledge_base/`; no embeddings/retriever |
| Next-best action rule | Rule-based scaffold; replace with agent output |
| Credit tier, contract end date, financing balance | Not in schema — shown as `—` |
| Usage telemetry (3-month avg, 5G %, roaming/intl flags) | `usage_telemetry.json` pending |
| Latency / grounding score | Static mock values |
| POS/Billing submission | In-memory mock log in session state |