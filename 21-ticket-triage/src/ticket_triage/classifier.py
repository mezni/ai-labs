import re


def classify_ticket(ticket_text: str) -> dict:
    """Classify a support ticket and extract relevant information."""
    if not isinstance(ticket_text, str) or not ticket_text.strip():
        raise ValueError("Ticket text must be a non-empty string.")

    text = ticket_text.lower()

    order_id_match = re.search(
        r"\border\s*#?\s*(\d+)\b",
        text,
    )
    order_id = order_id_match.group(1) if order_id_match else None

    _order_keywords = {
        "order",
        "delivery",
        "delivered",
        "shipment",
        "arrived",
        "shipping",
        "tracking",
    }

    _billing_keywords = {"refund", "charged", "billing", "payment"}

    _account_keywords = {"password", "login", "sign in", "account"}

    _technical_keywords = {"error", "bug", "crash", "technical"}

    if any(keyword in text for keyword in _order_keywords):
        category = "order_status"
    elif any(keyword in text for keyword in _billing_keywords):
        category = "billing"
    elif any(keyword in text for keyword in _account_keywords):
        category = "account"
    elif any(keyword in text for keyword in _technical_keywords):
        category = "technical"
    else:
        category = "general"

    return {
        "category": category,
        "order_id": order_id,
        "needs_order_lookup": category == "order_status" and order_id is not None,
    }
