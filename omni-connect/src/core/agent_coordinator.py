"""
Agent Coordinator.

Adapted from the bootcamp src/core/agent_coordinator.py.txt pattern: wires the
specialist agents into named steps and executes them against a customer,
aggregating each step's output into one combined workflow result.

omni-connect adaptations over the bootcamp original:
  - the specialist set is CustomerProfileAgent, PromotionEvaluatorAgent and
    PolicyRetrieverAgent (no PlannerAgent exists yet, so build_plan() is a
    deterministic default plan instead of a Planner-invoked sequence)
  - PolicyRetrieverAgent takes a question (not a customer_id), so its steps
    are encoded as "policy_retriever:<question>"
"""
from typing import Any, Dict, List, Optional

from src.agents.customer_profile_agent import CustomerProfileAgent
from src.agents.policy_retriever_agent import PolicyRetrieverAgent
from src.agents.promotion_evaluator_agent import PromotionEvaluatorAgent
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

DEFAULT_POLICY_QUESTION = "Check trade-in rules for returning devices"


class AgentCoordinator:
    """Wires the specialist agents into named steps, then executes a plan
    against a customer, aggregating each step's output into one combined
    workflow result."""

    def __init__(
        self,
        customer_profile_agent: Optional[CustomerProfileAgent] = None,
        promotion_evaluator_agent: Optional[PromotionEvaluatorAgent] = None,
        policy_retriever_agent: Optional[PolicyRetrieverAgent] = None,
    ) -> None:
        self.customer_profile_agent = customer_profile_agent or CustomerProfileAgent()
        self.promotion_evaluator_agent = promotion_evaluator_agent or PromotionEvaluatorAgent()
        self.policy_retriever_agent = policy_retriever_agent or PolicyRetrieverAgent()

        # Prefilled wiring: step name -> the bound agent method that executes
        # it. The policy retriever is handled specially in run_workflow because
        # its argument is a question rather than a customer_id.
        self.steps = {
            "customer_profile": self.customer_profile_agent.analyze,
            "promotion_evaluator": self.promotion_evaluator_agent.evaluate,
        }

    def build_plan(self, policy_questions: Optional[List[str]] = None) -> List[str]:
        """Return the workflow plan (list of step names).

        Profile first (no prior context), then promotion eligibility, then one
        policy_retriever step per question so later steps can consume the
        profile output as context.
        """
        questions = policy_questions or [DEFAULT_POLICY_QUESTION]
        plan = ["customer_profile", "promotion_evaluator"]
        plan.extend(f"policy_retriever:{q}" for q in questions)
        return plan

    def run_workflow(
        self,
        customer_id: str,
        policy_questions: Optional[List[str]] = None,
        plan: Optional[List[str]] = None,
    ) -> Dict[str, Any]:
        """Execute the workflow for `customer_id` and return one aggregated
        result dict keyed by step name."""
        plan = plan or self.build_plan(policy_questions=policy_questions)
        results: Dict[str, Any] = {}

        for step in plan:
            if step.startswith("policy_retriever:"):
                query = step.split(":", 1)[1]
                results["policy_retriever"] = self.policy_retriever_agent.analyze(
                    query,
                    context=results.get("customer_profile") or None,
                )
                continue

            agent_name = step
            if agent_name not in self.steps:
                raise KeyError(f"No coordinator step registered for agent '{agent_name}'")
            results[agent_name] = self.steps[agent_name](
                customer_id=customer_id, context=results
            )

        return results