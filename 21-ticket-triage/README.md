# AI Ticket Triage Agent

A cumulative learning project to design and build an AI-powered customer support ticket agent using plain Python, progressing from deterministic rule-based processing to LLM-powered reasoning, tool execution, orchestration, and evaluation.

The project emphasizes understanding agent architecture and engineering fundamentals by implementing the components directly rather than relying on agent frameworks.

## 1. Project Objectives

Build an agent that can:

- Accept and validate customer support tickets.
- Classify tickets by category and determine priority.
- Retrieve relevant support policies from a local knowledge base.
- Execute controlled Python tools to retrieve customer or order information.
- Orchestrate multi-step ticket processing.
- Generate context-aware responses using templates and, later, an LLM.
- Maintain agent state and persist ticket information.
- Apply safety rules and human approval for sensitive actions.
- Record execution history and evaluate system performance.

## 2. Problem Statement

Customer support teams receive tickets with different categories, urgency levels, and resolution requirements.

A ticket triage agent can help automate repetitive tasks, improve routing consistency, retrieve relevant information, and recommend appropriate responses.

The system must not blindly trust model outputs or allow unrestricted tool execution. It must validate inputs, enforce permissions, handle failures, and escalate cases that require human judgment.

## 3. Scope

### In scope

- Python-based ticket processing.
- Rule-based classification and prioritization.
- Local knowledge retrieval.
- Tool registration and execution.
- Agent orchestration and execution state.
- Optional LLM integration.
- Safety controls and human approval.
- Persistence, logging, testing, and evaluation.

### Out of scope initially

- Production customer data integrations.
- Autonomous financial transactions or refunds.
- Unrestricted execution of model-generated code.
- Agent frameworks such as LangChain, LangGraph, and CrewAI.
- Production deployment infrastructure.

These capabilities may be considered separately after the core application is reliable.

## 4. Technology Stack

| Technology | Purpose |
|---|---|
| Python 3.12+ | Core implementation |
| uv | Python project and environment management |
| Python standard library | JSON, logging, SQLite, HTTP requests, and core logic |
| `unittest` | Automated testing |
| JSON and text files | Sample tickets and support policies |
| SQLite | Persistent ticket data and execution history |
| Git and GitHub | Version control and documentation |
| LLM API or local model | Language-model inference in later stages |

The application will use plain Python and standard-library modules wherever practical. `uv` manages the development environment and project configuration; it is not an agent framework.

## 5. System Architecture

The target architecture separates ticket processing, agent control, execution tools, safety, and persistence.

See [docs/architecture.md](docs/architecture.md) for the complete Mermaid architecture diagram.

### Main components

| Component | Responsibility |
|---|---|
| Ticket Input | Receive ticket data |
| Validation | Validate fields and normalize input |
| Ticket Classifier | Determine the ticket category |
| Priority Engine | Estimate severity and urgency |
| Knowledge Retriever | Retrieve relevant support policies |
| Tool Executor | Validate and execute registered tools |
| Action Planner | Determine the next action |
| Agent Orchestrator | Coordinate components and control execution |
| Response Generator | Produce a response or recommendation |
| Safety Layer | Enforce permissions and approval requirements |
| Agent State | Maintain current execution context |
| Storage | Persist tickets and audit records |
| Evaluation | Measure quality, reliability, and performance |

### Execution flow

1. Receive and validate a ticket.
2. Initialize the agent execution state.
3. Classify the ticket and determine priority.
4. Retrieve relevant policies and information.
5. Select the next action through the orchestrator.
6. Execute permitted tools when needed.
7. Validate results and generate a response.
8. Request human approval for sensitive actions when required.
9. Persist the result and execution history.
10. Record evaluation metrics.

The initial implementation will use a deterministic workflow. More flexible LLM-based decisions will be introduced incrementally.

## 6. Learning Roadmap

The project is organized into six stages over approximately 30 development days.

### Stage 1 — Python Foundations

**Days 1–5**

- Ticket data structures and validation.
- Rule-based classification.
- Priority determination.
- Response generation.
- Unit testing and integration.

**Outcome:** A working rule-based ticket triage application.

### Stage 2 — Knowledge and Tools

**Days 6–10**

- Local support policy knowledge base.
- Keyword-based retrieval.
- Customer and order lookup tools.
- Tool registry and argument validation.
- Integration of tools and knowledge retrieval.

**Outcome:** An application that retrieves information and executes controlled operations.

### Stage 3 — Agent Architecture

**Days 11–15**

- Agent state and execution context.
- Orchestrator implementation.
- Task routing.
- Bounded execution loops.
- Retries, fallbacks, and explicit failure states.

**Outcome:** A multi-step agent workflow controlled by a custom Python orchestrator.

### Stage 4 — LLM Integration

**Days 16–20**

- Model inference and prompt design.
- LLM-based classification.
- Context-aware response generation.
- Model-selected tool calls.
- Structured output validation and failure handling.

**Outcome:** An LLM-powered agent with application-controlled tools and orchestration.

### Stage 5 — Reliability and Safety

**Days 21–25**

- Human approval and escalation.
- SQLite persistence.
- Conversation context and memory retrieval.
- Structured logging and audit history.
- Evaluation metrics and regression testing.

**Outcome:** A more reliable and auditable ticket agent.

### Stage 6 — Capstone and Engineering Quality

**Days 26–30**

- Batch ticket processing.
- Configuration management.
- Repeatable evaluation experiments.
- Security and failure testing.
- Architecture documentation and final review.

**Outcome:** A documented portfolio project with measurable results.

## 7. Repository Structure

The repository will grow incrementally as each stage is implemented.

```text
ticket-triage/
├── README.md
├── pyproject.toml
├── .gitignore
├── LOG.md
├── docs/
│   ├── architecture.md
│   ├── decisions.md
│   └── learning-log.md
├── data/
│   ├── sample_tickets.json
│   ├── policies.json
│   └── mock_orders.json
├── src/
│   └── ticket_triage/
│       ├── __init__.py
│       ├── ticket.py
│       ├── classifier.py
│       ├── priority.py
│       ├── response_generator.py
│       ├── knowledge_base.py
│       ├── retriever.py
│       ├── tools.py
│       ├── tool_executor.py
│       ├── state.py
│       ├── router.py
│       ├── orchestrator.py
│       ├── agent.py
│       ├── llm_client.py
│       ├── safety.py
│       ├── storage.py
│       ├── memory.py
│       ├── evaluation.py
│       └── main.py
└── tests/
    ├── test_ticket.py
    ├── test_classifier.py
    ├── test_priority.py
    ├── test_tools.py
    ├── test_orchestrator.py
    ├── test_agent.py
    └── test_safety.py
```

## 8. Development Principles

### Plain Python first

Implement the core logic using ordinary Python functions, classes where appropriate, and explicit control flow.

### Incremental development

Each step should extend the previous implementation. Avoid creating all components before the workflow is understood.

### Separation of concerns

Classification, prioritization, retrieval, tool execution, orchestration, safety, and persistence should have clearly defined responsibilities.

### Explicit control flow

The orchestrator controls which components run, the order of execution, and when the workflow terminates.

### Controlled tool execution

Tools must have explicit interfaces, validated arguments, and appropriate permissions. Model-generated requests must never bypass these checks.

### Testability

Every component should have testable inputs and outputs. Include normal cases, edge cases, and failure cases.

### Measurable quality

Evaluate classification quality, tool-selection correctness, policy relevance, response quality, latency, and failure rates.

### Documented decisions

Record implementation progress, technical friction, solutions, and trade-offs in `LOG.md`. Use `docs/decisions.md` for significant architectural decisions.

## 9. Development Tracking

Every development step should include:

1. **Component/Step:** The component being implemented.
2. **Goal:** The intended result.
3. **Problem/Friction:** The problem or technical challenge.
4. **Solution:** The implemented approach.
5. **Trade-offs:** Alternatives considered and their consequences.
6. **Status:** Not started, in progress, or completed.
7. **Next Step:** The next planned task.

Maintain this record in `LOG.md` and commit meaningful changes to Git.

### Commit message convention

Use Conventional Commits:

- `feat:` New functionality.
- `fix:` Bug fixes.
- `test:` Tests.
- `docs:` Documentation.
- `refactor:` Code restructuring.
- `chore:` Project maintenance.

Example:

```text
feat: implement rule-based ticket classification
```

## 10. Getting Started

### Prerequisites

- Python 3.12 or later.
- `uv`.
- Git.
- A code editor and terminal.

### Install uv

Follow the official installation instructions: https://docs.astral.sh/uv/getting-started/installation/

Verify the installation:

```bash
uv --version
```

### Clone the repository

```bash
git clone <your-repository-url>
cd ticket-triage
```

### Initialize the project

If the repository does not already contain a `pyproject.toml`, initialize it:

```bash
uv init --package --python 3.12
```

If `pyproject.toml` already exists, keep it and continue to the next step rather than initializing the project again.

### Create the virtual environment and lock dependencies

```bash
uv sync
```

`uv` creates the project environment and manages the lockfile. For this standard-library-first project, no additional runtime dependencies are initially required.

### Run the application

Once the entry point is implemented:

```bash
uv run python -m ticket_triage.main
```

### Run tests

```bash
uv run python -m unittest discover -s tests -v
```

### Add a dependency later

When a feature genuinely requires an external package:

```bash
uv add <package-name>
```

For a development-only dependency:

```bash
uv add --dev <package-name>
```

### Git workflow

Commit the project configuration and lockfile alongside your code:

```bash
git add .
git commit -m "chore: initialize uv project environment"
```

## 11. Definition of Done

The project is considered complete when it can:

- Process tickets from input through final result.
- Classify ticket categories and assign priorities.
- Retrieve relevant policies.
- Execute registered tools through a controlled executor.
- Orchestrate multi-step workflows using explicit state.
- Integrate an LLM without an agent framework.
- Handle invalid outputs, timeouts, and tool failures.
- Enforce permissions and human approval for sensitive operations.
- Persist tickets and execution history.
- Run repeatable tests and evaluation experiments.
- Document architecture, design decisions, limitations, and results.

## 12. Final Objective

Build a transparent, testable, and extensible AI ticket triage agent while developing a strong understanding of Python application engineering, LLM integration, agent orchestration, tool execution, safety controls, and system design.

The primary objective is not merely to make an agent work. It is to understand how and why each component works, how components interact, and how to evaluate the resulting system.