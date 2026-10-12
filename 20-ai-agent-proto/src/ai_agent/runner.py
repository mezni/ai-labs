from ai_agent.classifier import classify_ticket


def run_pipeline(raw_input: str) -> str:
    """Handles execution, input parsing, and response serialization."""
    result = classify_ticket(raw_input)
    return result.model_dump_json(indent=2)
