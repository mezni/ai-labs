from unittest.mock import patch

from support_agent.schemas import TicketOutput
from support_agent.workflow import run_workflow


def test_order_status_workflow():
    classification = TicketOutput(
        category="order_status",
        order_id="4821",
        needs_order_lookup=True,
    )

    with (
        patch(
            "support_agent.workflow.classify_ticket",
            return_value=classification,
        ),
        patch(
            "support_agent.workflow.get_order_status",
            return_value={
                "order_id": "4821",
                "status": "shipped",
                "estimated_delivery": "2026-10-12",
            },
        ) as order_lookup,
    ):
        state = run_workflow("Where is order #4821?")

    assert state.status == "completed"
    assert state.category == "order_status"
    assert state.order_id == "4821"
    assert "order_lookup" in state.completed_steps
    assert "2026-10-12" in state.answer
    order_lookup.assert_called_once_with("4821")


def test_order_status_without_id_requests_order_number():
    classification = TicketOutput(
        category="order_status",
        order_id=None,
        needs_order_lookup=False,
    )

    with (
        patch(
            "support_agent.workflow.classify_ticket",
            return_value=classification,
        ),
        patch("support_agent.workflow.get_order_status") as order_lookup,
    ):
        state = run_workflow("Where is my order?")

    assert state.status == "completed"
    assert "order number" in state.answer.lower()
    order_lookup.assert_not_called()


def test_refund_uses_knowledge_base_not_order_lookup():
    classification = TicketOutput(
        category="refund",
        order_id="4821",
        needs_order_lookup=False,
    )
    articles = [
        {
            "id": "returns-001",
            "title": "Return and Refund Policy",
            "content": "Returns can be requested within 30 days.",
        }
    ]

    with (
        patch(
            "support_agent.workflow.classify_ticket",
            return_value=classification,
        ),
        patch(
            "support_agent.workflow.search_knowledge_base",
            return_value=articles,
        ),
        patch("support_agent.workflow.get_order_status") as order_lookup,
    ):
        state = run_workflow("I want a refund for order #4821.")

    assert state.status == "completed"
    assert state.category == "refund"
    assert "30 days" in state.answer
    order_lookup.assert_not_called()


def test_empty_ticket_returns_failed_state():
    state = run_workflow("   ")

    assert state.status == "failed"
    assert state.error == "Workflow execution failed."
    assert state.answer is None


from support_agent.workflow import run_workflow


def main():
    ticket = input("Enter a support ticket: ")
    state = run_workflow(ticket)

    print(f"\nStatus: {state.status}")
    print(f"Current step: {state.current_step}")
    print(f"Completed steps: {state.completed_steps}")
    print(f"Category: {state.category}")
    print(f"Order ID: {state.order_id}")
    print(f"Answer: {state.answer}")

    if state.error:
        print(f"Error: {state.error}")


if __name__ == "__main__":
    main()
