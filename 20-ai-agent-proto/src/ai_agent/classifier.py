import logging
import re
from typing import Any
from ai_agent.models import (
    OperationalMetadata,
    RawTicketInput,
    TicketCategory,
    TicketClassification,
)

logger = logging.getLogger("ai_agent.classifier")


def classify_ticket_payload(payload: dict[str, Any]) -> TicketClassification:
    """Classifies incoming payloads according to a 5-step deterministic policy."""

    # Policy Rule 4: Invalid input is rejected by the schema (raises ValidationError on bad input)
    input_data = RawTicketInput.model_validate(payload)
    text = input_data.text
    text_lower = text.lower()

    # Extract metadata safely
    order_match = re.search(r"#?(\d{4,10})\b", text)
    order_id = order_match.group(1) if order_match else None

    # Construct operational metadata (Rule 5: No PII/raw text logged)
    meta = OperationalMetadata(
        text_length=len(text),
        word_count=len(text.split()),
        has_order_id_pattern=order_id is not None,
    )

    assumptions: list[str] = []

    # Keyword patterns
    has_refund_kw = any(kw in text_lower for kw in ["refund", "return", "chargeback", "money back"])
    has_order_kw = any(kw in text_lower for kw in ["order", "arrived", "shipping", "tracking", "delivered", "where is"])

    # --- DETERMINISTIC POLICY EVALUATION ---

    # Policy Rule 1: Refund requests take priority over order-status requests
    if has_refund_kw:
        category = TicketCategory.REFUND_DISPUTE
        confidence = 0.95 if order_id else 0.85
        assumptions.append("Policy Rule 1 Applied: Refund intent prioritized over order-status query.")
        if not order_id:
            assumptions.append("Refund requested without order_id.")

    # Policy Rule 2: Order-status requests may include a missing order ID
    elif has_order_kw or order_id:
        category = TicketCategory.ORDER_STATUS
        if order_id:
            confidence = 0.98
            assumptions.append("Policy Rule 2 Applied: Order status request matched with order ID.")
        else:
            confidence = 0.70
            assumptions.append("Policy Rule 2 Applied: Order status request identified without order ID.")

    # Policy Rule 3: Unknown requests fall back to general
    else:
        category = TicketCategory.GENERAL_INQUIRY
        confidence = 0.60
        assumptions.append("Policy Rule 3 Applied: Unmatched query defaulted to general inquiry.")

    # Operational routing flags
    needs_lookup = order_id is not None
    needs_human = confidence < 0.75 or (category == TicketCategory.REFUND_DISPUTE and not order_id)

    result = TicketClassification(
        category=category,
        confidence=confidence,
        order_id=order_id,
        needs_order_lookup=needs_lookup,
        needs_human_review=needs_human,
        assumptions=assumptions,
        metadata=meta,
    )

    # Policy Rule 5: Logs record operational metadata, NOT full customer message
    logger.info(
        "Ticket classified | category=%s confidence=%.2f order_id_present=%s "
        "needs_lookup=%s needs_human=%s text_length=%d word_count=%d",
        result.category.value,
        result.confidence,
        meta.has_order_id_pattern,
        result.needs_order_lookup,
        result.needs_human_review,
        meta.text_length,
        meta.word_count,
    )

    return result


def classify_ticket(text: str) -> TicketClassification:
    """Backward-compatible wrapper: classifies a raw ticket string."""
    return classify_ticket_payload({"text": text})