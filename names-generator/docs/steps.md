# Steps Tracker

Running log of steps as they are defined. Each time a new step is given, it is appended below.

## Process

```
<STEP>
<-- what to do -->
```

## Design (what we've done before)

| # | Step | Notes |
|---|---|---|
| 1 | Product discovery | docs/product.md + docs/architecture.md |
| 2 | Technical design (Feature 1: Name Generator) | docs/technical.md |

## Implementation

| # | Step | Notes |
|---|---|---|
| 0 | Prepare Python (python discovery) | Python 3.13, uv, venv, deps |
| 1 | Build a non-AI version | Rule-based name generator |
| 2 | Create the name-generation function | Topic → candidate names |
| 3 | Replace fake names with an LLM | OpenAI call in `generate_names` |
| 4 | Structured output | LLM returns `name`, `description`, `reason` per candidate |
| 5 | Refactor code | Move `main.py` into `app/` package, update entry point |
| 6 | Testing | Validation fixtures + pytest in `tests/` |
| 7 | Implementation | NameGenerator pipeline in `blueprint` |
| 8 | Telemetry and tracing | DIY JSONL spans (traces/), tokens + cost |

## Log

- **00 — Prepare Python:** Python 3.13, uv, venv, dependencies.
- **01 — Build a non-AI version:** rule-based name generator before wiring the LLM.
- **02 — Create the name-generation function:** generate candidate names from the project idea.
- **03 — Replace fake names with an LLM (done):** `generate_names` wired to OpenRouter (`minimax/minimax-m3:free`) via `OPENROUTER_API_KEY`, `.env` + `.env.example` added.
- **04 — Structured output:** LLM returns `{name, description, reason}` per candidate.
- **05 — Refactor code:** moved `main.py` into `app/` package; entry point now `app.main:main`.
- **01.01 — Product Discovery:** vision, problem, personas, user stories, roadmap.
- **01.02 — Technical Design:** API/interface, domain models, AI model, prompt design, output schema, validation, error handling, guardrails, evaluation, observability, deployment.
- **08 — Telemetry and tracing (done):** `utils.tracer` with nested spans → `traces/YYYY-MM-DD.jsonl` (trace_id, duration_ms, model, tokens, cost); toggle via `CATALYST_TRACING=0`.