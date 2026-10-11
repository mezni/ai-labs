from openai import APIConnectionError, APIStatusError, APITimeoutError

from support_agent.llm_client import create_llm_client
from support_agent.schemas import TicketInput, TicketOutput

SYSTEM_PROMPT = """
You classify customer support tickets.

Choose exactly one category:
- order_status: delivery, shipping, tracking, or order status
- refund: refunds, returns for reimbursement, or money back
- general: everything else

Decision rules:
1. If the customer requests a refund, choose refund,
   even when an order ID is included.
2. Extract an order ID only when explicitly present.
3. Never invent an order ID.
4. needs_order_lookup is true only when the category is
   order_status AND an order ID is present.
5. If the message asks about an order but gives no ID,
   classify it as order_status and set the lookup flag false.

Return data matching the supplied output schema.
""".strip()


def classify_ticket(ticket: str) -> TicketOutput:
    validated = TicketInput(message=ticket)
    client = create_llm_client()

    try:
        response = client.chat.completions.parse(
            model=__import__("os").environ["OPENAI_MODEL"],
            messages=[
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": validated.message},
            ],
            response_format=TicketOutput,
            temperature=0,
        )
    except (APITimeoutError, APIConnectionError):
        raise
    except APIStatusError:
        raise

    message = response.choices[0].message

    if message.refusal:
        raise ValueError("The model refused to classify this ticket.")

    if message.parsed is None:
        raise ValueError("The model returned no parsed result.")

    return message.parsed
