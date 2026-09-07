# RAG Tests

End-to-end Retrieval-Augmented Generation (RAG) copilot for **Aether Wireless** policies, with a measurable retrieval and generation evaluation harness.

## Stack

- **Embeddings & chat** — OpenRouter API (config in `config/llm_config.yaml`)
- **Vector store** — FAISS (`IndexFlatIP`, cosine over L2-normalized vectors)
- **Retrieval** — hybrid BM25 (sparse) + FAISS (dense) fused with Reciprocal Rank Fusion; optional cross-encoder reranker
- **Prompts** — versioned templates in `config/prompts.yaml`
- **Guardrails** — input, retrieval, and generation-grounding checks in `src/guardrails.py`
- **Memory** — conversation history + query rewriting in `src/memory.py`
- **Observability** — per-request JSONL logs in `src/observability.py`
- **UI** — Streamlit chat app

## Pipeline

One request flows through guardrails, conversational memory, retrieval, and
generation:

```
                    ┌───────────────────────────┐
   User question ──▶│ Input guardrail            │  reject empty question
                    └───────────┬───────────────┘
                                ▼
                    ┌───────────────────────────┐
                    │ Query processing          │  rewrite against history  (memory)
                    │  (Conversation history)   │  → standalone query
                    └───────────┬───────────────┘
                                ▼
                    ┌───────────────────────────┐
                    │ Retriever (BM25 + FAISS)  │  hybrid RRF, optional rerank
                    └───────────┬───────────────┘
                                ▼
                    ┌───────────────────────────┐
                    │ Retrieval guardrail        │  low similarity → "No relevant information was found."
                    └───────────┬───────────────┘
                                ▼
                    ┌───────────────────────────┐
                    │ LLM (history + context)   │  grounded generation (RAG)
                    └───────────┬───────────────┘
                                ▼
                    ┌───────────────────────────┐
                    │ Grounding guardrail        │  ungrounded answer → refused
                    └───────────┬───────────────┘
                                ▼
                          Answer  ──▶  Observability log (data/logs)
```

```
src/
  knowledge/      EmbeddingGenerator, BM25Index, Reranker, KnowledgeBase (FAISS), Retriever
  llm/            LLMClient (OpenRouter), PromptManager (multi-version)
  evaluation/     metrics, retrieval, generation
  guardrails.py   input / retrieval / grounding guardrails
  memory.py       conversation memory + query rewriting
  observability.py request logger
  utils/          config loader, logging
scripts/          generate_docs, rag_pipeline_runner, eval_retrieval, eval_generation
app/              streamlit_app.py
data/
  policies/       source markdown corpus
  faiss/          built index (index.bin + chunks.json)
  validation/     eval_questions.json
  logs/           per-request observability output (jsonl + readable .log)
```

## Setup

```bash
cp .env.example .env   # set OPENROUTER_API_KEY
uv sync
```

## Generate the corpus & build the index

```bash
uv run python scripts/generate_docs.py     # writes data/policies/*.md
# index is built lazily by the runner/app, or force it:
uv run python scripts/rag_pipeline_runner.py --query "How long is the return window?" --rebuild
```

## Query

```bash
uv run python scripts/rag_pipeline_runner.py --query "How much does a 5 GB Data Boost cost?" --rerank
uv run python scripts/rag_pipeline_runner.py --query "How much does a 5 GB Data Boost cost?" --with-memory --history "User: Tell me about data boosts\nAssistant: Data boosts add monthly data."
uv run python scripts/rag_pipeline_runner.py --query "..." --min-similarity 0.5
uv run streamlit run app/streamlit_app.py
```

## Conversation memory

Conversation memory and knowledge retrieval are different things:

- **Memory** answers *"what were we talking about?"* — the conversation history.
- **RAG** answers *"what does the knowledge base say?"* — the retrieved corpus.

With memory enabled the pipeline becomes:

```
User → Conversation history → Query processing → Retriever → LLM → Answer
```

`src/memory.py` implements both sides:

1. **`ConversationMemory`** — a bounded rolling history (last `--max-turns`
   user/assistant pairs). It only records what was said; it never retrieves
   knowledge.
2. **`QueryProcessor`** — strips the history offline (no retrieval) and rewrites
   the current question into a *standalone* retrieval query, resolving pronouns
   and references ("how much is **it**?", "and the return window?"). It uses
   the `query_rewrite` prompt; if the LLM is unavailable the original question
   is used unchanged.

The rewritten standalone query drives the retriever (RAG side). The history is
also injected into the generation prompt (`{history}` in `retrieval_query`), so
the answer side can resolve references — but the system prompt still requires
grounding in the retrieved context, not the history.

```bash
# CLI: seed history; "how much is it?" is rewritten into a standalone question
uv run python scripts/rag_pipeline_runner.py --query "how much is it?" --with-memory \
  --history "User: What does a 5 GB Data Boost cost?\nAssistant: It costs 5 per month."
```

The Streamlit app keeps the transcript in `session_state`, rewrites each new
question against it, and includes history in generation. `Clear conversation`
resets memory. Every request logs `history` and `rewritten_query` (see
Observability).

## Evaluation

Questions and expected answers/sources live in `data/validation/eval_questions.json`.

### Retrieval evaluation — did we retrieve the right information?

Metrics: **Recall@K**, **Precision@K**, **Hit@K** (hit rate), **MRR@K**.

```bash
uv run python scripts/eval_retrieval.py --ks 1,3,5
uv run python scripts/eval_retrieval.py --ks 5 --json-out results/retrieval.json
```

A question is relevant to the chunks that share its `expected_source`; scores are averaged across the dataset.

### Generation evaluation — was the answer correct, grounded, hallucination-free?

The generation evaluation answers three questions about each produced answer:

1. **Was the answer correct?**
   - **Embedding similarity** — cosine similarity between the answer and the expected answer embeddings (`similarity >= threshold` counts as correct).
   - **LLM-as-judge** (optional) — a judge prompt (`generation_eval`) scores correctness on `0.0–1.0`.

2. **Was it grounded in the retrieved context?**
   - **Lexical groundedness** — mean fraction of the answer's sentences whose token set is covered by the retrieved context.
   - **LLM-as-judge** — a `grounded` boolean from the judge.

3. **Did it hallucinate?**
   - **Hallucination ratio** — fraction of answer sentences with no lexical support in the context.
   - **LLM-as-judge** — a `hallucinated` boolean from the judge.

```bash
# lexical + embedding evaluation only (fast, no extra chat calls)
uv run python scripts/eval_generation.py

# add LLM-as-judge for correctness/grounded/hallucinated verdicts
uv run python scripts/eval_generation.py --judge --json-out results/generation.json
```

## Tests

```bash
uv run pytest                           # unit tests (offline)
RUN_INTEGRATION=1 uv run pytest -m integration   # live OpenRouter tests
```

## Observability

Every pipeline request is recorded automatically (runner + Streamlit app) by
`src/observability.py::RequestLogger` to `data/logs/rag_requests.jsonl`, with a
human-readable view in `data/logs/rag_requests.log`. Each record captures:

- `request_id` & `timestamp`
- `question`
- `history` (conversation memory used) and `rewritten_query` (standalone retrieval query)
- `retrieved_chunks` (id, source, score) and `retrieval_scores`
- `prompt` (the messages sent to the model)
- `model`
- `latency` (seconds)
- `usage` (prompt/completion/total tokens from the API response)
- `answer`
- `sources` (deduplicated source files)
- `error` (set when generation fails)
- `guardrails` — per-stage check outcomes (`stage`, `passed`/blocked, `message`)

```text
REQUEST 0318e833d4eb

Question:
How much is it?

Rewritten query:
How much does a 5 GB Data Boost cost?

Conversation history:
User: Tell me about data boosts.
Assistant: Data boosts add monthly data.

Retrieval:
  chunk_003  score=0.910
  chunk_008  score=0.840

Model:
gpt-...

Latency:
1.43s

Token usage:
  prompt_tokens=100 | completion_tokens=40 | total_tokens=140

Answer:
A 5 GB Data Boost costs $5.00 per month.

Sources:
pricing/data-boosts.md

Guardrails:
  input: passed
  retrieval: passed
  generation: passed
```

The JSONL format is machine-readable for dashboards/aggregation; the `.log` view
is for quick debugging.

## Guardrails

`src/guardrails.py` enforces three stages; thresholds live in `config/llm_config.yaml`:

| Stage | Guardrail | When blocked |
| ----- | --------- | ------------ |
| Input | `validate_query()` | Empty/whitespace question is rejected (`Question must not be empty.`) |
| Retrieval | `check_retrieval()` | Top chunk's dense similarity < `guardrails.retrieval.min_similarity` (0.30) → answers `No relevant information was found.` |
| Generation | `check_grounding()` | Answer's lexical groundedness in retrieved context < `guardrails.generation.min_groundedness` (0.30) → refuses the ungrounded answer |

```bash
uv run python scripts/rag_pipeline_runner.py --query "..." --min-similarity 0.5
```

The retriever exposes `dense_similarity` (raw cosine from FAISS) on each chunk so
the retrieval threshold reflects true similarity, independent of RRF/rerank score scales.
