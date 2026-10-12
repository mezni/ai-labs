def assess_priority(ticket_text: str) -> dict:
    """Determine ticket priority using deterministic keyword rules."""
    if not isinstance(ticket_text, str) or not ticket_text.strip():
        raise ValueError("Ticket text must be a non-empty string.")

    text = ticket_text.lower()

    critical_keywords = (
        "production is down",
        "entire service is down",
        "service outage",
        "system-wide outage",
        "production service is down",
    )

    high_keywords = (
        "cannot access my account",
        "payment failed",
        "cannot renew",
        "security breach",
        "account compromised",
        "data breach",
    )

    medium_keywords = (
        "charged twice",
        "duplicate charge",
        "order has not arrived",
        "refund",
        "delivery delayed",
    )

    if any(keyword in text for keyword in critical_keywords):
        priority = "critical"
        reason = "Major service disruption detected."
    elif any(keyword in text for keyword in high_keywords):
        priority = "high"
        reason = "Urgent account, payment, or security issue detected."
    elif any(keyword in text for keyword in medium_keywords):
        priority = "medium"
        reason = "Issue may require investigation or follow-up."
    else:
        priority = "low"
        reason = "No predefined high-urgency indicators detected."

    return {
        "priority": priority,
        "reason": reason,
    }
