# RAG DataOps Pipeline

LlamaIndex + ChromaDB + UV + OpenRouter + Pydantic

A DataOps-oriented Retrieval-Augmented Generation (RAG) pipeline: heterogeneous document ingestion, deterministic change detection, document metadata and lineage, and versioned vector indexes that can be reproduced and rolled back.

## Status

**Design phase.** The architecture is specified in [docs/PROJECT.md](docs/PROJECT.md); the implementation task list is in [docs/TODO.md](docs/TODO.md). Nothing is implemented yet — this README describes the intended design.

## What It Does

- **Ingests documents** from heterogeneous sources: **PDF, DOCX, websites, Confluence, SharePoint, and databases**
- **Hashes documents** (SHA-256) for idempotent change detection — NEW / UNCHANGED / CHANGED / DELETED
- **Tracks document versioning** — hash-derived versions (e.g. `refund-policy.pdf` → `v4`)
- **Records metadata and lineage** — every chunk knows its document, version, source connector, and index snapshot
- **Chunks documents** through LlamaIndex `NodeParser` with configurable size and overlap
- **Embeds** chunks through a configurable embedding model (OpenRouter gateway), tracking model, provider, and dimension per snapshot
- **Stores vectors and chunk metadata in ChromaDB** — the single vector store
- **Versions indexes as full snapshots** — each index build is an immutable, rollback-able Chroma snapshot catalogued in an append-only registry
- **Rolls back** retrieval to any previously built snapshot by repointing the current version
- **Retrieves** top-k chunks with lineage metadata
- **Visualizes DataOps** in a Streamlit dashboard — catalog, lineage, index versions, and one-click rollback

## RAGOps Maturity Phases

The pipeline is delivered in three verifiable phases (see [docs/TODO.md](docs/TODO.md)):

| Phase | Capability | What you learn |
|-------|-----------|---------------|
| RAGOps 1 | Document hashing | Change detection, idempotency |
| RAGOps 2 | Document metadata | Lineage and provenance |
| RAGOps 3 | Index versioning | Reproducibility + rollback |

## Tech Stack

| Layer        | Technology                                            |
|--------------|-------------------------------------------------------|
| Language     | Python 3.12+                                          |
| Packaging    | UV                                                   |
| RAG framework| LlamaIndex (NodeParser, `ChromaVectorStore`, retriever) |
| Vector store | ChromaDB (`PersistentClient`), versioned snapshots   |
| Ingestion    | `pypdf`, `python-docx`, `beautifulsoup4`, LlamaIndex readers |
| Validation   | Pydantic + `pydantic-settings` (config)              |
| Model access | OpenRouter (OpenAI-compatible client, `openai`)       |
| Dashboard    | Streamlit (multipage, `app/`)                        |
| Config       | `python-dotenv` (via `pydantic-settings`)             |

## Project Structure

```
02-rag-dataops/
├── data/raw/                    # filesystem documents to index
├── indexes/
│   ├── chroma/                  # ChromaDB PersistentClient (vectors + chunk metadata)
│   ├── index_registry.json      # index version catalog + "current" pointer
│   └── document_catalog.json    # document hashes, versions, metadata, lineage
├── logs/rag-dataops.log
├── docs/                        # PROJECT.md, TODO.md, decisions/ (ADRs)
├── scripts/
│   └── generate_sample_docs.py  # regenerate sample source documents
├── app/                         # Streamlit dashboard (app.py + pages/, artifacts.py)
│   ├── app.py                   # Overview page + entrypoint
│   └── pages/                   # Documents, Index Versions, Lineage, Search, Rollback
├── src/
│   ├── config.py                # pydantic-settings
│   ├── main.py                  # rag-dataops CLI
│   ├── ingestion/               # loader.py, parser.py, hashing.py
│   ├── chunking/                # chunker.py
│   ├── embeddings/              # embedder.py
│   ├── indexing/                # builder.py, registry.py, versioning.py
│   └── retrieval/               # retriever.py
└── tests/                       # pytest unit tests
```

Full layout with module responsibilities: [docs/PROJECT.md](docs/PROJECT.md) §4.

## Getting Started

```bash
cp .env.example .env                     # set OPENROUTER_API_KEY, OPENROUTER_EMBEDDING_MODEL
uv sync                                  # install dependencies into .venv
uv run python scripts/generate_sample_docs.py   # generate sample documents into data/raw
uv run ruff check src tests              # lint
uv run pytest                            # run the test suite
```

## CLI

The `rag-dataops` entrypoint (`src/main.py`) will provide:

```bash
rag-dataops index                     # discover, hash, change-detect, build snapshot, bump version
rag-dataops rollback <version>        # point current_version back to a stored snapshot
rag-dataops versions                  # list index versions + configuration
rag-dataops catalog                   # show document catalog (versions, hashes, lineage)
rag-dataops search "<query>"          # top-k chunks with lineage metadata
```

## Streamlit Dashboard

A multipage Streamlit dashboard (`app/`) visualizes the DataOps artifacts and is the browser front end for rollback:

```bash
uv run streamlit run app/app.py
```

| Page | Content |
|------|---------|
| Overview | current index version, chunk/document counts, per-source distribution |
| Documents | document catalog: versions, hashes, metadata, lineage |
| Index Versions | registry versions + configuration (embedding model/dimension, chunking) |
| Lineage | chunk → document → version → source connector → index snapshot |
| Search | top-k retrieval with lineage metadata |
| Rollback | repoint `current_version` to any stored snapshot (confirmed) |

The dashboard reads `indexes/index_registry.json` and `indexes/document_catalog.json` and calls the same services as the CLI — it never performs indexing. Spec: [docs/PROJECT.md](docs/PROJECT.md) §20.

## Roadmap

- RAGOps 2 hardening (richer metadata extraction, lineage UX)
- RAGOps 3 hardening (snapshot retention policy, async snapshot build)
- Retrieval metrics (Recall@k, MRR, Precision@k)
- Hybrid search (semantic + keyword), reranking, evaluation harness, guardrails
- FastAPI endpoints wrapping the same services
- Production connectors for Confluence / SharePoint / databases

See [docs/PROJECT.md](docs/PROJECT.md) §22 for the full roadmap.