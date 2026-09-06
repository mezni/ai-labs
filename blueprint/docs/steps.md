# Steps Tracker

Running log of steps as they are defined. Each time a new step is given, it is appended below.

## Process

```
<STEP>
<-- what to do -->
```

## Design (what we've done before)

| # | Step | Status | Notes |
|---|---|---|---|
| 1 | Product discovery | Done | docs/product.md + docs/architecture.md |
| 2 | Technical design (Feature 1: Name Generator) | Done | docs/technical.md |

## Implementation

| # | Step | Status | Notes |
|---|---|---|---|
| 0 | Prepare Python (python discovery) | Pending | Python 3.13, uv, venv, deps |
| 1 | Build a non-AI version | Pending | Rule-based name generator |
| 3 | Implementation | Pending | NameGenerator pipeline in `blueprint` |
| 4 | Testing | Pending | Validation fixtures + pytest in `tests/` |

## Log

- **00 — Prepare Python:** Python 3.13, uv, venv, dependencies.
- **01 — Build a non-AI version:** rule-based name generator before wiring the LLM.
- **01.01 — Product Discovery:** vision, problem, personas, user stories, roadmap.
- **01.02 — Technical Design:** API/interface, domain models, AI model, prompt design, output schema, validation, error handling, guardrails, evaluation, observability, deployment.