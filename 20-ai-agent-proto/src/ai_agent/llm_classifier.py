import logging
from typing import Any
from openai import OpenAI
from ai_agent.models import (
    LLMTicketClassification,
    RawTicketInput,
    TicketClassification,
    OperationalMetadata,
)

logger = logging.getLogger("ai_agent.llm_classifier")

SYSTEM_PROMPT = """You are an expert customer support ticket classifier.
Analyze the user's message and categorize it into exactly one of the following:
- refund_dispute: Customer wants money back, return, or chargeback.
- order_status: Customer is asking about delivery, shipping status, or package location.
- technical_support: Customer is experiencing technical issues or bugs.
- general_inquiry: Questions about store hours, policies, or general info.

Policy Rules:
1. Refund requests ALWAYS take priority over general order queries.
2. If an order ID (e.g., #4821 or 4821) is mentioned, extract it cleanly as digits only.
"""

def classify_ticket_with_llm(
    payload: dict[str, Any], client: Any | None = None
) -> TicketClassification:
    """Classifies a customer ticket using an LLM with enforced structured outputs."""
    # 1. Validate raw input payload against schema
    input_data = RawTicketInput.model_validate(payload)
    text = input_data.text

    if client is None:
        try:
            from openai import OpenAI

            client = OpenAI()
        except Exception:
            logger.error("OpenAI client not available — falling back to keyword classifier")
            from ai_agent.classifier import classify_ticket_payload

            return classify_ticket_payload(payload)

    # 2. Call LLM with structured output constraint
    response = client.beta.chat.completions.parse(
        model="gpt-4o-mini",
        messages=[
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": text},
        ],
        response_format=LLMTicketClassification,
        temperature=0.0,
    )

    llm_out: LLMTicketClassification = response.choices[0].message.parsed

    # 3. Compute operational metadata & policy flags
    meta = OperationalMetadata(
        text_length=len(text),
        word_count=len(text.split()),
        has_order_id_pattern=llm_out.order_id is not None,
    )

    needs_human = (
        llm_out.confidence < 0.75
        or (llm_out.category.value == "refund_dispute" and not llm_out.order_id)
    )

    logger.info(
        "LLM Classification Complete | category=%s confidence=%.2f order_id=%s needs_human=%s",
        llm_out.category.value,
        llm_out.confidence,
        llm_out.order_id,
        needs_human,
    )

    return TicketClassification(
        category=llm_out.category,
        confidence=llm_out.confidence,
        order_id=llm_out.order_id,
        needs_order_lookup=llm_out.order_id is not None,
        needs_human_review=needs_human,
        assumptions=[f"LLM Reasoning: {llm_out.reasoning}"],
        metadata=meta,
    )