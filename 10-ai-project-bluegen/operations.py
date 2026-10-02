from dataclasses import dataclass


@dataclass(frozen=True)
class Operation:
    event_type: str
    name: str

    def __post_init__(self) -> None:
        if not self.event_type.strip():
            raise ValueError("event_type cannot be empty.")

        if not self.name.strip():
            raise ValueError("Operation name cannot be empty.")


# Application operations

APPLICATION_GENERATE_BLUEPRINT = Operation(
    event_type="application",
    name="generate_blueprint",
)


# Service operations

SERVICE_GENERATE_BLUEPRINT = Operation(
    event_type="service",
    name="generate_blueprint",
)


# LLM operations

LLM_PROJECT_BLUEPRINT_GENERATION = Operation(
    event_type="llm",
    name="project_blueprint_generation",
)


# Future agent operation

AGENT_EXECUTION = Operation(
    event_type="agent",
    name="agent_execution",
)


# Future tool operations

TOOL_KNOWLEDGE_SEARCH = Operation(
    event_type="tool",
    name="knowledge_search",
)


# Future retrieval operations

RETRIEVAL_SEARCH = Operation(
    event_type="retrieval",
    name="retrieval_search",
)


# Future database operations

DATABASE_QUERY = Operation(
    event_type="database",
    name="database_query",
)


# Future evaluation operations

EVALUATION_RUN = Operation(
    event_type="evaluation",
    name="evaluation_run",
)


class Operations:
    APPLICATION_GENERATE_BLUEPRINT = APPLICATION_GENERATE_BLUEPRINT
    SERVICE_GENERATE_BLUEPRINT = SERVICE_GENERATE_BLUEPRINT

    LLM_PROJECT_BLUEPRINT_GENERATION = (
        LLM_PROJECT_BLUEPRINT_GENERATION
    )

    AGENT_EXECUTION = AGENT_EXECUTION

    TOOL_KNOWLEDGE_SEARCH = TOOL_KNOWLEDGE_SEARCH

    RETRIEVAL_SEARCH = RETRIEVAL_SEARCH

    DATABASE_QUERY = DATABASE_QUERY

    EVALUATION_RUN = EVALUATION_RUN
