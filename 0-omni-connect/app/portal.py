"""
Omni-Connect Retail Copilot Portal - Streamlit UI.

Empty application scaffold. Wire the business-data services (src/services/),
the knowledge base (data/knowledge_base/), and the recommendation pipeline
into this portal as the starter app matures. Mirrors the bootcamp
streamlit_app.py conventions (system path wiring + page config).

Run:  streamlit run app/portal.py
"""
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

import streamlit as st

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from src.services import billing_service, catalog_service, customer_service, promotion_service


def format_tenure(created_at: str) -> str:
    created = datetime.fromisoformat(created_at)
    now = datetime.now(timezone.utc)
    years = (now - created).days / 365.25
    if years >= 1:
        return f"{max(1, round(years))} Years"
    months = max(1, round((now - created).days / 30.44))
    return f"{months} Months"


def render_field_grid(fields: list[tuple[str, str, str | None]], per_row: int = 2) -> None:
    for start in range(0, len(fields), per_row):
        row = fields[start:start + per_row]
        cols = st.columns(len(row))
        for col, (label, value, tip) in zip(cols, row):
            with col:
                st.caption(label)
                st.markdown(f"**{value}**")
                if tip:
                    st.caption(tip)


def render_subsection(title: str, fields: list[tuple[str, str, str | None]]) -> None:
    st.divider()
    st.markdown(f'<div class="profile-header">{title}</div>', unsafe_allow_html=True)
    render_field_grid(fields)


PROMPT_SHORTCUTS = [
    "Compare with competitor X",
    "Check trade-in rules",
    "Explain price change",
]


def load_kb_snippet(filename: str, max_chars: int = 220) -> dict | None:
    """Thin knowledge-base citation loader: pulls the document ID, version,
    and a short excerpt from a curated policy document in data/knowledge_base/.
    Stands in for the RAG retriever until the pipeline is wired."""
    path = Path("data/knowledge_base") / filename
    if not path.exists():
        return None
    text = path.read_text(encoding="utf-8")
    doc_id = re.search(r"\*\*Document ID:\*\*\s+(\S+)", text)
    version = re.search(r"\*\*Version:\*\*\s+([0-9.]+)", text)
    title = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
    excerpt = ""
    in_overview = False
    for line in text.splitlines():
        if line.strip() == "## Overview":
            in_overview = True
            continue
        if in_overview and line.strip() and not line.startswith("#") and not line.strip() == "---":
            excerpt = " ".join(line.strip().split())[:max_chars]
            break
    return {
        "doc_id": doc_id.group(1) if doc_id else "?",
        "version": version.group(1) if version else "?",
        "title": title.group(1) if title else filename,
        "filename": filename,
        "excerpt": excerpt,
    }


def add_copilot_message(role: str, content: str) -> None:
    st.session_state.copilot_messages.append({"role": role, "content": content})


def recommend_next_best_action(profile: dict, plan: dict, device: dict) -> dict:
    """Scaffold recommendation rule: top structured next-best action + monthly
    financial delta for the selected customer. Stands in for the real
    recommendation agent until the pipeline is wired."""
    if profile["account_status"] == "Delinquent":
        return {
            "title": "Settle balance, then offer win-back",
            "detail": "Clear the outstanding balance, then re-qualify for PROMO-0005 "
                      "(Unlimited at $75/mo).",
            "monthly_delta": None,
        }
    catalog_plans = catalog_service.load_business_data("product_catalog.json")["plans"]
    tier_ladder = ["Essential", "Standard", "Premium", "Unlimited"]
    current_name = plan["name"]
    next_name = None
    if current_name in tier_ladder:
        idx = tier_ladder.index(current_name)
        if idx + 1 < len(tier_ladder):
            next_name = tier_ladder[idx + 1]
    if next_name is None:
        return {
            "title": "Keep current plan",
            "detail": "Customer is already on the top tier; suggest add-ons instead.",
            "monthly_delta": 0.0,
        }
    next_plan = next(p for p in catalog_plans if p["name"] == next_name)
    if profile["account_status"] == "Prospect":
        return {
            "title": f"Activate new line — {next_plan['name']}",
            "detail": f"Recommended line setup for the prospect on {next_plan['name']}, "
                      f"paired with the {device['model']}.",
            "monthly_delta": next_plan["monthly_price"],
        }
    return {
        "title": f"Upgrade to {next_plan['name']} + {device['model']} (trade-in)",
        "detail": "Matched on recent usage and line profile; trade-in credit applies at "
                  "point of sale.",
        "monthly_delta": next_plan["monthly_price"] - plan["monthly_price"],
    }

st.set_page_config(
    page_title="Omni-Connect Retail Copilot Portal",
    page_icon="📡",
    layout="wide",
    initial_sidebar_state="expanded",
)

st.markdown(
    """
<style>
    .main-header {font-size: 2.4rem; color: #1f4788; font-weight: bold;}
    .sub-header {font-size: 1.1rem; color: #555; margin-bottom: 1.5rem;}
    .profile-header {font-weight: bold; color: #1f4788; margin-bottom: 0.4rem;}
    .footer {position: fixed; left: 0; bottom: 0; width: 100%; background-color: #f5f5f5;
             text-align: center; padding: 0.5rem 1rem; font-size: 0.85rem; color: #777;
             border-top: 1px solid #ddd; z-index: 100;}
</style>
""",
    unsafe_allow_html=True,
)

st.markdown('<div class="main-header">📡 Omni-Connect Retail Copilot Portal</div>', unsafe_allow_html=True)
st.markdown(
    '<div class="sub-header">Retail copilot for plan, device, billing, and promotion recommendations</div>',
    unsafe_allow_html=True,
)

with st.sidebar:
    st.header("Platform Status")
    st.info("Empty scaffold - platform components not wired yet.")
    st.divider()
    st.caption("Omni-Connect Retail Copilot · Starter App")

st.markdown('<div class="sub-header">Workspace</div>', unsafe_allow_html=True)

catalog_col, promotion_col, billing_col = st.columns(3)

with catalog_col:
    with st.container(border=True):
        st.subheader("Customer")

        customers = customer_service.load_business_data("crm_records.json")
        customer_ids = [record["customer_id"] for record in customers]
        selected_customer = st.selectbox(
            "Customer",
            customer_ids,
            key="customer_selectbox",
            label_visibility="collapsed",
        )

        profile = customer_service.get_customer(selected_customer)

        st.markdown('<div class="profile-header">Profile Header</div>', unsafe_allow_html=True)
        profile_cols = st.columns(4)
        profile_fields = [
            ("Customer ID", profile["customer_id"], None),
            ("Tenure", format_tenure(profile["created_at"]), "from account creation"),
            ("Account Status", profile["account_status"], "Active / Delinquent / Prospect"),
            ("Credit Tier", "—", "not in schema yet"),
        ]
        for col, (label, value, tip) in zip(profile_cols, profile_fields):
            with col:
                st.caption(label)
                st.markdown(f"**{value}**")
                if tip:
                    st.caption(tip)

        billing = billing_service.get_billing_account(profile["billing_account_id"])
        plan = catalog_service.get_plan(profile["current_plan_id"])
        device = catalog_service.get_device(billing["device_id"])

        render_subsection(
            "Line Details",
            [
                ("Line ID", profile["contact"]["phone"], "line identifier (MDN)"),
                ("Current Rate Plan", plan["name"], plan["plan_id"]),
                ("Contract End Date", "—", "not in schema yet"),
                ("Device Model", device["model"], device["device_id"]),
                ("Remaining Financing Balance", "—", "not in schema yet"),
            ],
        )

        telemetry = [
            ("3-Month Avg Data Usage (GB)", "—"),
            ("5G Usage %", "—"),
        ]
        render_subsection(
            "Usage Telemetry",
            [
                *((label, value, "usage_telemetry.json pending") for label, value in telemetry),
                ("Roaming Call Flag", "—", "usage_telemetry.json pending"),
                ("International Call Flag", "—", "usage_telemetry.json pending"),
            ],
        )

        invoices = billing["invoices"]
        monthly_bill_average = sum(inv["total_amount"] for inv in invoices) / len(invoices)
        overdue_count = sum(1 for inv in invoices if inv["status"] == "OVERDUE")
        if overdue_count == 0:
            reliability = "Reliable"
        elif overdue_count <= 2:
            reliability = "Occasionally Late"
        else:
            reliability = "At Risk"

        render_subsection(
            "Billing Context",
            [
                ("Monthly Bill Average", f"${monthly_bill_average:,.2f}", "last 6 invoices"),
                ("Auto-Pay Status", "Enabled" if billing["autopay_enabled"] else "Disabled", None),
                ("Payment Reliability", reliability, f"{overdue_count}/{len(invoices)} recent invoices overdue"),
            ],
        )

with promotion_col:
    with st.container(border=True):
        st.subheader("Agent")
        st.caption("realtime copilot · RAG · dynamic queries")

        if "copilot_messages" not in st.session_state:
            st.session_state.copilot_messages = [
                {
                    "role": "assistant",
                    "content": (
                        f"Hi! I'm your omni-connect copilot for the selected customer. "
                        "I can recommend plans, check promo eligibility, explain charges, "
                        "and ground every answer in company policy."
                    ),
                }
            ]

        # -- Promotional Eligibility Cards
        promotions = promotion_service.load_business_data("promotions.json")
        eligible_promotions = [
            promo for promo in promotions
            if promo["promotion_id"] in profile["eligible_promotion_ids"]
        ]
        st.markdown('<div class="profile-header">Promotional Eligibility</div>', unsafe_allow_html=True)
        if not eligible_promotions:
            st.caption("No offers currently eligible for this account.")
        for promo in eligible_promotions:
            with st.container(border=True):
                st.markdown(f"**{promo['title']}**")
                st.markdown(promo["description"])
                st.caption(
                    f"{promo['promo_type']} · ${promo['discount_amount']:,.2f} · "
                    f"valid until {promo['valid_until'][:10]}"
                )

        # -- RAG Knowledge Snippets (expandable source citations)
        source_docs = [
            "promotions_eligibility_terms.md.txt",
            "plan_upgrade_policy.md.txt",
            "billing_invoice_policy.md.txt",
        ]
        if any(promo["promotion_id"] == "PROMO-0003" for promo in eligible_promotions):
            source_docs.append("device_trade_in_process.md.txt")
        st.markdown('<div class="profile-header">RAG Knowledge Snippets</div>', unsafe_allow_html=True)
        for filename in source_docs:
            snippet = load_kb_snippet(filename)
            if snippet is None:
                continue
            with st.expander(f"{snippet['doc_id']} · {snippet['title']}"):
                st.markdown(f"*{snippet['filename']}* · v{snippet['version']}")
                st.markdown(snippet["excerpt"])
        st.caption("Competitor battlecards: not authored in the knowledge base yet.")

        # -- Quick Prompt Shortcuts
        st.markdown('<div class="profile-header">Quick Prompts</div>', unsafe_allow_html=True)
        shortcut_cols = st.columns(len(PROMPT_SHORTCUTS))
        for col, shortcut in zip(shortcut_cols, PROMPT_SHORTCUTS):
            with col:
                if st.button(shortcut, use_container_width=True):
                    add_copilot_message("user", shortcut)
                    add_copilot_message(
                        "assistant",
                        f"Looking into *{shortcut}* for {profile['customer_id']}. "
                        "Copilot rationale will stream here once RAG retrieval and the "
                        "recommendation pipeline are wired.",
                    )

        # -- Copilot Chat Stream
        st.markdown('<div class="profile-header">Copilot Chat</div>', unsafe_allow_html=True)
        for message in st.session_state.copilot_messages:
            with st.chat_message(message["role"]):
                st.markdown(message["content"])
        if prompt := st.chat_input("Ask the copilot — e.g. 'which plans fit this usage?'"):
            add_copilot_message("user", prompt)
            add_copilot_message(
                "assistant",
                f"Context loaded for **{profile['customer_id']}** ({plan['name']}, "
                f"{billing['device_id']}). Full recommendation pending agent wiring — "
                "this is the scaffold response.",
            )

with billing_col:
    with st.container(border=True):
        st.subheader("Decision")
        st.caption("action capture · overrides · order submission")

        recommendation = recommend_next_best_action(profile, plan, device)

        # -- Recommended Next-Best Action + Financial Delta
        st.markdown('<div class="profile-header">Recommended Next-Best Action</div>', unsafe_allow_html=True)
        with st.container(border=True):
            st.markdown(f"**{recommendation['title']}**")
            st.markdown(recommendation["detail"])
            delta = recommendation["monthly_delta"]
            if delta is None:
                st.markdown("**Financial Delta:** balance due")
            else:
                sign = "+" if delta > 0 else ""
                st.markdown(f"**Financial Delta:** {sign}${delta:,.2f}/mo")
                st.caption("after any applicable trade-in credit (illustrative)")

        # -- Action State (Accept / Override / Reject)
        st.markdown('<div class="profile-header">Action State</div>', unsafe_allow_html=True)
        action_state = st.radio(
            "Action",
            ["Accept", "Override", "Reject"],
            horizontal=True,
            key="action_state",
        )

        override_reason = None
        if action_state == "Override":
            override_reason = st.selectbox(
                "Override Reason",
                ["Price Resistance", "Customer Prefers Legacy Plan", "Device Not Available", "Other"],
                index=None,
                placeholder="Select a required override reason...",
                key="override_reason",
            )

        # -- System Telemetry Summary
        st.markdown('<div class="profile-header">System Telemetry</div>', unsafe_allow_html=True)
        st.markdown("**Latency:** 1.2s  |  **Grounding Score:** 0.96")
        st.caption("mock telemetry — wire observability instrumentation later")

        # -- Primary Action Button
        submit_disabled = action_state == "Override" and not override_reason
        if st.button("Submit Order to POS", type="primary", disabled=submit_disabled, key="submit_order"):
            order = {
                "submitted_at": datetime.now(timezone.utc).isoformat(),
                "customer_id": profile["customer_id"],
                "action": recommendation["title"],
                "action_state": action_state,
                "override_reason": override_reason,
                "latency_ms": 1200,
                "grounding_score": 0.96,
            }
            st.session_state.setdefault("pos_orders", []).append(order)
            st.success(f"Order submitted to POS/Billing mock pipeline as **{action_state}**.")
        if submit_disabled:
            st.caption("Override requires a reason before submission.")

        if st.session_state.get("pos_orders"):
            with st.expander(f"Recently Submitted Orders ({len(st.session_state['pos_orders'])})"):
                for order in st.session_state["pos_orders"][-5:]:
                    suffix = f" · reason: **{order['override_reason']}**" if order["override_reason"] else ""
                    st.markdown(
                        f"- **{order['submitted_at'][:19]}Z** — {order['customer_id']} · "
                        f"{order['action_state']} · {order['action']}{suffix}"
                    )

st.markdown(
    '<div class="footer">© 2026 Omni-Connect Retail Copilot · Demo data is synthetic and generated by scripts/</div>',
    unsafe_allow_html=True,
)