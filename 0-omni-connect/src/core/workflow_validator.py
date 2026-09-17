"""
Workflow Validator.

Adapted from the bootcamp src/core/workflow_validator.py.txt pattern:
structural, in-workflow validation of an AgentCoordinator.run_workflow()
result, before it is treated as a finished recommendation. Not a
comprehensive test suite - that's for the test layer; this is the platform
checking its own output shape at runtime.

omni-connect adaptations over the bootcamp original:
  - required_result_fields come from config/agent_config.yaml's
    validation section (customer_profile, promotion_evaluator,
    policy_retriever)
  - a section that returned an error payload (e.g. unknown customer) is
    treated as invalid, since the coordinator's result would otherwise look
    structurally complete but carry no retrievable content
  - the bootcamp allowed_decisions check is dropped: omni-connect has no
    recommendation decision vocabulary yet
"""
from typing import Any, Dict, List, Optional

from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)


class WorkflowValidator:
    """Validates the shape of an aggregated workflow result before it is
    returned to a caller (e.g. the Streamlit portal)."""

    def __init__(self) -> None:
        config = load_yaml_config("config/agent_config.yaml")
        self.rules = config.get("validation", {})

    def validate(self, result: Dict[str, Any]) -> Optional[List[str]]:
        """
        Check `result` (the dict AgentCoordinator.run_workflow() returns)
        against the expected shape. Return a list of human-readable error
        strings if anything is wrong, or None if `result` is valid.
        """
        errors: List[str] = []
        required_fields = self.rules.get("required_result_fields", [])

        for name in required_fields:
            section = result.get(name)
            if not isinstance(section, dict):
                errors.append(f"Missing or invalid '{name}' section")
            elif "error" in section:
                errors.append(f"'{name}' section returned an error: {section.get('error')}")

        return errors if errors else None