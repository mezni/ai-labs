# RAG Tests

End-to-end Retrieval-Augmented Generation (RAG) copilot for **Aether Wireless** policies, with a measurable retrieval and generation evaluation harness.

## Stack

- **Embeddings & chat** — OpenRouter API (config in `config/llm_config.yaml`)
- **Vector store** — FAISS (`IndexFlatIP`, cosine over L2-normalized vectors)
- **Retrieval** — hybrid BM25 (sparse) + FAISS (dense) fused with Reciprocal Rank Fusion; optional cross-encoder reranker
- **Prompts** — versioned templates in `config/prompts.yaml`
- **UI** — Streamlit chat app

```
src/
  knowledge/      EmbeddingGenerator, BM25Index, Reranker, KnowledgeBase (FAISS), Retriever
  llm/            LLMClient (OpenRouter), PromptManager (multi-version)
  evaluation/     metrics, retrieval, generation
  utils/          config loader, logging
scripts/          generate_docs, rag_pipeline_runner, eval_retrieval, eval_generation
app/              streamlit_app.py
data/
  policies/       source markdown corpus
  faiss/          built index (index.bin + chunks.json)
  validation/     eval_questions.json
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
uv run streamlit run app/streamlit_app.py
```

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
