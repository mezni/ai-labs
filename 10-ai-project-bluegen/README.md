# AI Project Blueprint Generator

An agentic AI learning project that transforms a simple AI project idea into a structured, implementation-ready architecture blueprint.

The project is built progressively with **Python, Pydantic, LangChain, and OpenRouter**, starting with a simple structured LLM application and evolving toward a production-oriented agentic AI system.

---

## 1. Project Goal

The goal is to build an AI architecture assistant that can take a high-level project idea such as:

```text
Build an AI system that classifies corporate documents
as Public, Internal, Confidential, or Secret.
```

and transform it into a structured architecture blueprint.

The final system will reason about:

* Business outcome
* Requirements
* Boundaries
* Constraints
* Agents
* Orchestration
* Context
* State
* Memory
* Knowledge
* Actions
* Authority
* Runtime
* Recovery
* Evaluation
* Observability
* Security
* Responsible AI

The project is also a practical learning journey for **AI engineering from design to production**.

---

## 2. Learning Philosophy

The system will **not** start as a complex multi-agent application.

It will evolve incrementally:

```text
Structured LLM
      ↓
Single Blueprint Generator
      ↓
Pipeline
      ↓
Agentic Architecture
      ↓
Tools
      ↓
RAG
      ↓
State & Memory
      ↓
Human-in-the-Loop
      ↓
Recovery
      ↓
Evaluation
      ↓
Observability
      ↓
Security
      ↓
CI/CD
      ↓
FinOps
      ↓
Production
```

At each stage, the objective is to understand:

```text
Concept
   ↓
Implementation
   ↓
Failure Modes
   ↓
Evaluation
   ↓
Production Considerations
```

---

## 3. First Feature

The first functional feature is intentionally small.

### Input

```text
Build an AI system that classifies corporate documents.
```

### Output

```text
Project Name:
Corporate Document Classification System

Business Outcome:
Automatically classify corporate documents into appropriate
security categories to improve document handling and reduce
manual classification effort.
```

This establishes the foundation for the larger blueprint generator.

---

## 4. Technology Stack

### Core

* Python
* Pydantic
* LangChain
* OpenRouter

### Configuration

* Python environment variables
* `.env`

### Testing

* pytest

### Future

* LangChain pipelines
* Agent orchestration
* Tool calling
* RAG
* Vector database
* FastAPI
* Streamlit
* Docker
* CI/CD
* Observability
* Evaluation
* FinOps

---

## 5. Architecture Evolution

### Level 1 — Structured LLM

The first version is intentionally simple:

```text
User
 │
 ▼
Project Idea
 │
 ▼
Prompt
 │
 ▼
OpenRouter LLM
 │
 ▼
Pydantic Validation
 │
 ▼
Structured Blueprint
```

The initial learning topics are:

* Prompt engineering
* LangChain
* OpenRouter
* Pydantic
* Structured output

---

### Level 2 — Pipeline

The generator will eventually be decomposed into stages:

```text
Project Idea
     │
     ▼
Requirements Analysis
     │
     ▼
Architecture Design
     │
     ▼
Security Analysis
     │
     ▼
Architecture Critique
     │
     ▼
Final Blueprint
```

Learning topics:

* LCEL
* Chains
* Pipeline composition
* State
* Decomposition
* Validation

---

### Level 3 — Agentic Architecture

Specialized responsibilities will eventually become agents:

```text
                  User
                    │
                    ▼
              Orchestrator
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
Requirements   Architecture   Security
   Agent          Agent         Agent
       │            │            │
       └────────────┼────────────┘
                    ▼
               Critic Agent
                    │
                    ▼
             Blueprint Builder
                    │
                    ▼
             Final Blueprint
```

The system will only introduce agents where autonomous reasoning provides value.

Deterministic operations should remain normal application code.

---

## 6. AI vs Deterministic Code

A core architectural principle is:

> Use AI for ambiguity and reasoning. Use deterministic code for rules and guarantees.

### LLM responsibilities

Examples:

* Requirements analysis
* Architecture reasoning
* Risk analysis
* Architecture critique
* Business outcome generation

### Python responsibilities

Examples:

* Schema validation
* Authentication
* Authorization
* Permission enforcement
* Saving data
* Deterministic calculations
* Rule enforcement
* Tool access control

This separation prevents unnecessary use of LLMs for tasks that can be guaranteed by normal software.

---

## 7. Future Tools

Agents will eventually have access to controlled tools such as:

```text
search_architecture_patterns()
retrieve_security_policy()
retrieve_reference_architecture()
search_company_standards()
estimate_cost()
validate_blueprint()
save_blueprint()
request_human_review()
```

Tools will have explicit permissions and authority boundaries.

---

## 8. Knowledge and RAG

A future version will introduce a knowledge layer.

Potential knowledge sources:

```text
security-policies
architecture-standards
ai-governance
reference-architectures
cloud-standards
data-governance
ai-operations
```

The architecture will evolve toward:

```text
Documents
    │
    ▼
Document Loader
    │
    ▼
Chunking
    │
    ▼
Embeddings
    │
    ▼
Vector Store
    │
    ▼
Retriever
    │
    ▼
Architecture Agent
```

This allows agents to ground architecture decisions in authoritative information.

---

## 9. Context Design

Agents should receive only the context required for their task.

Potential context:

```text
User Request
Project Requirements
Current Blueprint
Relevant Policies
Retrieved Knowledge
Previous Agent Results
Available Tools
```

The entire project history should not automatically be placed into every prompt.

---

## 10. State, Memory and Knowledge

These concepts will be treated separately.

### State

Temporary information required by the current workflow.

Examples:

```text
current architecture
current review
current evaluation score
revision count
```

### Memory

Information intentionally retained across interactions.

Examples:

```text
user architecture preferences
preferred cloud platform
```

### Knowledge

External authoritative information retrieved when needed.

Examples:

```text
company policies
security standards
reference architectures
```

---

## 11. Authority

Agents will have explicit authority boundaries.

For example:

### Architecture Agent

Can:

```text
Propose architecture
Recommend technologies
Retrieve architecture patterns
```

Cannot:

```text
Deploy infrastructure
Modify production systems
Approve security exceptions
Access unrelated sensitive information
```

The project will apply least-privilege principles.

---

## 12. Human-in-the-Loop

High-risk actions should be able to stop for human approval.

```text
Agent
  │
  ▼
Proposed Action
  │
  ▼
Risk Assessment
  │
  ├── Low Risk ──────► Execute
  │
  └── High Risk ─────► Human Approval
                           │
                    ┌──────┴──────┐
                    ▼             ▼
                 Approve        Reject
                    │             │
                    ▼             ▼
                 Execute         Stop
```

---

## 13. Recovery

Agentic systems must expect failure.

The future system will handle cases such as:

```text
LLM Failure
    ↓
Retry
    ↓
Fallback Model
    ↓
Human Escalation
```

Other failure scenarios include:

```text
Invalid structured output
    ↓
Schema validation
    ↓
Retry

Tool unavailable
    ↓
Retry
    ↓
Fallback

Knowledge unavailable
    ↓
Fail safely

Too many revisions
    ↓
Human review
```

---

## 14. Evaluation

The generated blueprint should be evaluated rather than blindly accepted.

Potential evaluation dimensions:

* Requirements completeness
* Architecture quality
* Security coverage
* Constraint coverage
* Grounding
* Hallucination rate
* Tool-use correctness
* Citation correctness
* Business outcome quality

A future evaluation dataset will contain representative project ideas and expected characteristics.

---

## 15. Observability

The production system will eventually capture:

```text
Request ID
Model
Prompt version
Input
Output
Retrieved knowledge
Tool calls
Latency
Token usage
Errors
Retries
Evaluation results
Human overrides
```

This makes the system measurable and debuggable.

---

## 16. Security and Responsible AI

The system will eventually address:

* Sensitive information protection
* Access control
* Least privilege
* Prompt injection
* Data leakage
* Audit logging
* Human approval
* Output validation
* Agent authority
* Tool authorization
* Responsible AI controls

Security controls should be implemented primarily through deterministic application mechanisms rather than relying solely on prompts.

---

## 17. Learning Roadmap

## Phase 1 — Foundation

Learn:

* Python project structure
* `uv`
* Virtual environments
* `.env`
* Git
* Dependencies

Build:

```text
Project foundation
```

---

## Phase 2 — Pydantic

Learn:

* `BaseModel`
* Fields
* Type hints
* Validation
* Serialization
* Nested models

Build:

```text
Blueprint schema
```

---

## Phase 3 — OpenRouter

Learn:

* LLM APIs
* API keys
* Models
* Temperature
* Tokens
* Configuration

Build:

```text
OpenRouter LLM connection
```

---

## Phase 4 — Prompt Engineering

Learn:

* System prompts
* User prompts
* Instructions
* Constraints
* Output requirements
* Few-shot examples

Build:

```text
Project idea → LLM response
```

---

## Phase 5 — Structured Output

Learn:

* LangChain structured output
* Pydantic integration
* Output parsing
* Validation failures

Build:

```text
Project idea
     ↓
Structured Blueprint
```

---

## Phase 6 — Project Name + Business Outcome

Build the first real feature:

```text
Project Idea
     ↓
Project Name
     ↓
Business Outcome
```

Learn:

* Information extraction
* Structured generation
* Output quality
* Validation

---

## Phase 7 — Complete Blueprint

Add:

```text
Boundaries
Constraints
Agents
Orchestration
Context
Memory
Knowledge
Actions
Authority
Runtime
Recovery
Evaluation
Observability
Security
```

---

## Phase 8 — Testing

Learn:

* pytest
* Unit tests
* Integration tests
* Mocking
* Regression tests
* LLM test cases

---

## Phase 9 — LangChain Pipeline

Learn:

* LCEL
* Chains
* Composition
* Sequential processing
* State

Build:

```text
Requirements
      ↓
Architecture
      ↓
Security
      ↓
Critique
      ↓
Blueprint
```

---

## Phase 10 — Agentic AI

Learn:

* Agent definition
* Agent vs chain
* Tool selection
* Autonomous reasoning
* Agent loops
* Orchestration

Build specialized agents.

---

## Phase 11 — Tools

Learn:

* Tool schemas
* Function calling
* Tool selection
* Tool authorization
* Tool errors

Build deterministic tools.

---

## Phase 12 — RAG

Learn:

* Document loading
* Chunking
* Embeddings
* Vector stores
* Retrieval
* Context injection
* Grounding

---

## Phase 13 — State and Memory

Learn:

* Workflow state
* Persistent memory
* Memory boundaries
* Context management

---

## Phase 14 — Human-in-the-Loop

Learn:

* Approval workflows
* Risk thresholds
* Escalation
* Human review

---

## Phase 15 — Recovery

Learn:

* Retries
* Fallbacks
* Timeouts
* Validation failures
* Safe failure
* Recovery workflows

---

## Phase 16 — Evaluation

Learn:

* Evaluation datasets
* LLM evaluation
* Deterministic metrics
* Regression testing
* Quality gates

---

## Phase 17 — Observability

Learn:

* Structured logging
* Metrics
* Tracing
* Request IDs
* Token tracking
* Latency monitoring

---

## Phase 18 — Security

Learn:

* Prompt injection
* Authorization
* Least privilege
* Data protection
* Agent boundaries
* Tool security
* Auditability

---

## Phase 19 — API and UI

Learn:

* FastAPI
* API schemas
* Service architecture
* Streamlit
* Frontend/backend separation

---

## Phase 20 — Production

Learn:

* Docker
* CI/CD
* Deployment
* Configuration management
* Production monitoring
* Cost management
* FinOps

---

## 18. Initial Project Structure

Start intentionally small.

The current (Level 1) structure:

```text
ai-project-bluegen/
│
├── app.py                  # launcher → cli.main()
├── cli.py                  # replaceable CLI presentation layer
├── application.py          # Application + create_application (composition root, creates RequestContext + TraceContext)
├── container.py            # DependencyContainer (assembles infrastructure, never runs use cases)
├── context.py              # RequestContext (frozen request_id, trace_id, create_span_id)
├── service.py              # ProjectBlueprintService (use case; receives the TraceContext)
├── interfaces.py           # ProjectGenerator/StructuredLLM/PromptManager/TelemetryRecorder interfaces
├── generator.py            # ProjectGenerator (validation, telemetry, prompts; opens spans via the trace)
├── structured_llm.py       # LangChainStructuredLLM (ChatOpenAI + structured output)
├── prompt_manager.py       # PromptManager (versioned prompt files)
├── logger.py               # StructuredLogger (request_id/operation fields)
├── tracing.py              # TraceContext (start_span/finish_span) — emits TelemetryEvent with trace/span/parent ids
├── schemas.py              # ProjectBlueprint + typed request/response models
├── exceptions.py
├── telemetry.py            # GenerationTelemetry, TelemetryEvent (trace/span), GenerationResult
├── logging_config.py       # configure_logging() + RequestContextFilter
├── config.py               # Settings (env) + LLMConfig (yaml) — the configuration boundary
├── config/
│   ├── llm.yaml
│   └── prompts.yaml
├── prompts/
│   └── project_blueprint/
│       └── v1/             # system.txt, user.txt
├── tests/
│   ├── fakes.py
│   └── test_*.py
│
├── .env
├── .env.example
├── .gitignore
├── pyproject.toml
├── uv.lock
├── CHANGELOG.md
├── docs/
│   ├── ROADMAP.md
│   ├── STEPS.md
│   └── SESSION_HANDOFF.md
└── README.md
```

The structure grows only when the corresponding architectural concept is introduced. See `docs/STEPS.md` for the granular step-by-step progression (each step's architectural choice and why).

---

## 19. Long-Term Structure

The eventual architecture may evolve toward:

```text
blueprint-generator/
│
├── agents/
│   ├── requirements_agent.py
│   ├── architecture_agent.py
│   ├── security_agent.py
│   └── critic_agent.py
│
├── tools/
│   ├── policy_search.py
│   ├── architecture_search.py
│   └── validator.py
│
├── workflows/
│   └── blueprint_graph.py
│
├── knowledge/
├── memory/
├── evaluation/
├── observability/
├── api/
│
├── schemas.py
├── prompts.py
├── config.py
├── app.py
│
├── tests/
│
├── Dockerfile
├── docker-compose.yml
├── pyproject.toml
└── README.md
```

This structure is intentionally **not** created all at once.

---

## 20. Success Criteria

The project is successful when it can:

1. Accept a natural-language AI project idea.
2. Generate a meaningful project name.
3. Define the business outcome.
4. Produce a structured architecture.
5. Separate AI responsibilities from deterministic code.
6. Decompose complex work into appropriate stages.
7. Use specialized agents when justified.
8. Use controlled tools.
9. Ground decisions in retrieved knowledge.
10. Maintain explicit workflow state.
11. Apply authority boundaries.
12. Support human approval for high-risk operations.
13. Recover from common failures.
14. Evaluate generated blueprints.
15. Provide useful observability.
16. Apply security and Responsible AI controls.
17. Run through an API/UI.
18. Be testable and deployable.
19. Track operational cost.
20. Be suitable as a foundation for a production AI engineering system.

---

## 21. Development Principle

The central principle of this project is:

```text
Start simple.
Understand the failure.
Measure the result.
Add complexity only when it solves a real problem.
```

The goal is not simply to build an agent.

The goal is to understand how to design, build, evaluate, secure, observe, operate, and deploy **production-grade AI systems**.