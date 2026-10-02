# Learning Roadmap

> Granular, increment-by-increment progress (with each step's architectural choice and why) is tracked in [STEPS.md](./STEPS.md). This roadmap is the high-level 42-step curriculum.

| Step | Build | What you learn | Status |
| --- | --- | --- | --- |
| 1 | Project foundation | Python project structure, uv, virtual environments, .env, Git | ✅ |
| 2 | Pydantic Blueprint schema | Pydantic, type safety, validation, structured data | ✅ |
| 3 | OpenRouter LLM client | LLM APIs, models, temperature, tokens, configuration | ✅ |
| 4 | First simple prompt | Prompt engineering, system vs user prompts | ✅ |
| 5 | Structured LLM output | LangChain structured output + Pydantic | ✅ |
| 6 | Project Name + Business Outcome | Your first real feature; extracting useful structured information from an idea | ✅ |
| 7 | Complete single-agent blueprint | Generate boundaries, constraints, agents, knowledge, actions, etc. | ⬜ |
| 8 | Prompt engineering v2 | Better instructions, constraints, few-shot examples, output quality | ⬜ |
| 9 | Validation layer | Pydantic validation + deterministic Python validation | ⬜ |
| 10 | Tests | Unit tests, integration tests, test cases for LLM applications | ✅ |
| 11 | LCEL pipeline | LangChain Expression Language, chains, composition | ⬜ |
| 12 | Multi-stage pipeline | Requirements → Architecture → Security → Critic → Blueprint | ⬜ |
| 13 | Shared state | TypedDict, workflow state, passing results between stages | ⬜ |
| 14 | Critic/revision loop | AI evaluation, iterative generation, revision | ⬜ |
| 15 | First real agentic version | Agent vs chain vs pipeline; when an LLM should make decisions | ⬜ |
| 16 | Specialized agents | Requirements Agent, Architecture Agent, Security Agent, Critic Agent | ⬜ |
| 17 | Orchestrator | Routing, delegation, agent coordination | ⬜ |
| 18 | Tool calling | Function/tool schemas, tool selection, controlled actions | ⬜ |
| 19 | Deterministic tools | Validators, cost calculator, architecture checker | ⬜ |
| 20 | Authority boundaries | Permissions, least privilege, allowed/forbidden actions | ⬜ |
| 21 | Human-in-the-loop | Approval gates, high-risk actions, escalation | ⬜ |
| 22 | Recovery | Retry, fallback model, invalid output recovery, failure handling | ◐ retry + backoff + recovery decision loop wired (0.0.46–0.0.52); fallback model and invalid-output recovery pending |
| 23 | Observability | Logs, request IDs, traces, latency, token usage, errors | ✅ |
| 24 | Evaluation | Accuracy, completeness, relevance, hallucination, structured evals | ⬜ |
| 25 | Evaluation dataset | Golden examples, expected outputs, regression testing | ⬜ |
| 26 | Knowledge/RAG | Documents → chunks → embeddings → vector store → retrieval | ⬜ |
| 27 | Architecture knowledge base | Security policies, architecture standards, reference architectures | ⬜ |
| 28 | Retrieval tools | Agents querying authoritative knowledge | ⬜ |
| 29 | Context engineering | What information each agent receives and why | ⬜ |
| 30 | State vs memory vs knowledge | Workflow state, persistent memory, external knowledge | ⬜ |
| 31 | Memory | User/project preferences and cross-interaction persistence | ⬜ |
| 32 | Security | Prompt injection, sensitive data, access control, data leakage | ⬜ |
| 33 | Responsible AI | Risk identification, human oversight, auditability | ⬜ |
| 34 | Production API | FastAPI, request/response models, service architecture | ⬜ |
| 35 | UI | Streamlit interface for the blueprint generator | ⬜ |
| 36 | Configuration | YAML/config management, dev/test/prod settings | ⬜ |
| 37 | CI/CD | GitHub Actions, tests, linting, deployment pipeline | ⬜ |
| 38 | Containerization | Docker, environment configuration | ⬜ |
| 39 | Production telemetry | Tracing, metrics, dashboards, LLM monitoring | ⬜ |
| 40 | Cost / FinOps | Token usage, model cost, cost per blueprint, optimization | ◐ |
| 41 | Production evaluation | Automated quality gates before accepting a blueprint | ⬜ |
| 42 | Final production architecture | Put everything together into a production-grade agentic system | ⬜ |

Legend: ✅ completed · ◐ in progress · ⬜ not started

## The important progression

Don't jump directly to the multi-agent architecture in your document. Follow this evolution:

```text
                    LEVEL 1
             Structured LLM
                    │
                    ▼
        Project Idea → Blueprint
                    │
                    ▼
                    LEVEL 2
                  Pipeline
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
   Requirements  Architecture  Security
        │           │           │
        └───────────┼───────────┘
                    ▼
                  Critic
                    │
                    ▼
                    LEVEL 3
               Agentic System
                    │
              ┌─────┴─────┐
              ▼           ▼
            Agents       Tools
              │           │
              └─────┬─────┘
                    ▼
                   RAG
                    │
                    ▼
             State / Memory
                    │
                    ▼
          Human-in-the-Loop
                    │
                    ▼
          Evaluation + Recovery
                    │
                    ▼
              Observability
                    │
                    ▼
             CI/CD + FinOps
                    │
                    ▼
             PRODUCTION
```

This progression matches the document's principle of not immediately building a complex multi-agent system and instead evolving the architecture progressively.

## What I recommend for your learning

Since your goal is learning AI engineering from design → production, don't treat this as just a project to finish.

At every step, learn three things:

```text
1. CONCEPT
   ↓
2. IMPLEMENTATION
   ↓
3. FAILURE MODES
```

For example:

### Step 6 — Project Name + Business Outcome

You learn:

```text
LLM
 └── Prompt
      └── Structured Output
           └── Pydantic
                └── Validation
```

Then deliberately test:

```text
vague project descriptions
incomplete descriptions
contradictory requirements
very long descriptions
hallucinated business outcomes
malformed LLM responses
```

That is where the real AI-engineering learning happens.

## Your first milestone

I would make Steps 1–6 your first milestone:

Input:

```text
"Build an AI system that classifies corporate documents."
```

Output:

```text
Project Name:
Corporate Document Classification System

Business Outcome:
Automatically classify corporate documents into
appropriate security categories to improve document
handling and reduce manual classification effort.
```

Once that works reliably, we move to the full blueprint.