from dataclasses import dataclass, field
from typing import Any, Literal

Route = Literal[
    "order_lookup",
    "knowledge_search",
    "clarify_order_id",
]

@dataclass
class WorkflowState:
    ticket: str
    status: Literal["pending", "running", "completed", "failed"] = "pending"
    current_step: str = "not_started"
    completed_steps: list[str] = field(default_factory=list)
    category: str | None = None
    order_id: str | None = None
    tool_results: dict[str, Any] = field(default_factory=dict)
    answer: str | None = None
    error: str | None = None


from support_agent.llm_classifier import classify_ticket
from support_agent.tools import get_order_status
from support_agent.knowledge_base import search_knowledge_base
from support_agent.schemas import TicketInput


def _mark_step(state: WorkflowState, step: str) -> None:
    state.current_step = step
    state.completed_steps.append(step)

def route_workflow(
    category: str,
    order_id: str | None,
    needs_order_lookup: bool,
) -> Route:
    if category == "order_status":
        if needs_order_lookup and order_id is not None:
            return "order_lookup"
        return "clarify_order_id"

    if category in {"refund", "general"}:
        return "knowledge_search"

    raise ValueError(f"Unsupported ticket category: {category}")

def run_workflow(ticket: str) -> WorkflowState:
    state = WorkflowState(ticket=ticket, status="running")

    try:
        state.current_step = "validate"
        validated = TicketInput(message=ticket)
        state.ticket = validated.message
        _mark_step(state, "validate")

        state.current_step = "classify"
        classification = classify_ticket(state.ticket)
        state.category = classification.category
        state.order_id = classification.order_id
        _mark_step(state, "classify")

        if classification.needs_order_lookup:
            state.current_step = "order_lookup"
            result = get_order_status(classification.order_id)
            state.tool_results["order_status"] = result
            _mark_step(state, "order_lookup")

            if result["status"] == "not_found":
                state.answer = (
                    f"I couldn't find order #{classification.order_id}. "
                    "Please check the order number and try again."
                )
            else:
                state.answer = (
                    f"Order #{classification.order_id} is "
                    f"{result['status']}. "
                    f"Estimated delivery: {result['estimated_delivery']}."
                )

        elif classification.category == "order_status":
            state.answer = (
                "I can help check your order status. Please provide your order number."
            )

        else:
            state.current_step = "knowledge_search"
            articles = search_knowledge_base(state.ticket)
            state.tool_results["knowledge_search"] = articles
            _mark_step(state, "knowledge_search")

            if articles:
                # Keep retrieved information grounded in the article text.
                state.answer = "\n\n".join(
                    f"{article['title']}: {article['content']}" for article in articles
                )
            else:
                state.answer = (
                    "I couldn't find an answer in the available support documentation."
                )

        state.status = "completed"
        state.current_step = "completed"
        return state

        route = route_workflow(
            category=classification.category,
            order_id=classification.order_id,
            needs_order_lookup=classification.needs_order_lookup,
        )

        state.current_step = "route"
        state.tool_results["selected_route"] = route
        _mark_step(state, "route")

        if route == "order_lookup":
            state.current_step = "order_lookup"
            result = get_order_status(classification.order_id)
            state.tool_results["order_status"] = result
            _mark_step(state, "order_lookup")

            if result["status"] == "not_found":
                state.answer = (
                    f"I couldn't find order #{classification.order_id}. "
                    "Please check the order number and try again."
                )
            else:
                state.answer = (
                    f"Order #{classification.order_id} is "
                    f"{result['status']}. "
                    f"Estimated delivery: {result['estimated_delivery']}."
                )

        elif route == "clarify_order_id":
            state.answer = (
                "I can help check your order status. "
                "Please provide your order number."
            )

        elif route == "knowledge_search":
            state.current_step = "knowledge_search"
            articles = search_knowledge_base(state.ticket)
            state.tool_results["knowledge_search"] = articles
            _mark_step(state, "knowledge_search")

            if articles:
                state.answer = "\n\n".join(
                    f"{article['title']}: {article['content']}"
                    for article in articles
                )
            else:
                state.answer = (
                    "I couldn't find an answer in the available "
                    "support documentation."
                )

        state.status = "completed"
        state.current_step = "completed"
        return state

    except Exception:
        # Log the exception internally in a production application.
        state.status = "failed"
        state.error = "Workflow execution failed."
        state.current_step = "failed"
        return state
