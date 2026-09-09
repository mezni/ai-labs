# TODO

Implementation tasks for the RAG DataOps pipeline (LlamaIndex + ChromaDB).
Reference: [PROJECT.md](PROJECT.md).

## Phase 0 — Scaffolding & Configuration

- [x] Initialize project with UV and add `llama-index-core`, `llama-index-vector-stores-chroma`, `chromadb`, `pydantic`, `pydantic-settings`, `openai`, `pypdf`, `python-docx`, `beautifulsoup4`, `python-dotenv`
- [x] Add `streamlit` as a `dashboard` extra in `pyproject.toml` (not a core runtime dependency)
- [x] Create `pyproject.toml` with Python 3.12, ruff, and pytest dev dependencies
- [x] Add `.env.example` and `.gitignore` (`.env`, `data/raw/`, `data/processed/`, `indexes/`, `logs/`)
- [x] Create directory skeleton: `data/raw/`, `indexes/chroma/`, `logs/`, `src/{config,main}.py`, `src/ingestion/`, `src/chunking/`, `src/embeddings/`, `src/indexing/`, `src/retrieval/`, `tests/`
- [x] Implement `src/config.py` with OpenRouter, Chroma, registry/catalog paths, chunking, and TOP_K settings
- [x] Write ADR-001: LlamaIndex + ChromaDB as the RAG stack (vs. pure-Python FAISS)
- [x] Write ADR-002: full snapshots per index version (physical rollback), append-only registry
- [x] Add sample documents to `data/raw/` for development (`scripts/generate_sample_docs.py`)

## RAGOps 1 — Document Hashing (change detection + idempotency)

### Hashing (`src/ingestion/hashing.py`)

- [x] Implement streaming SHA-256 hashing of document content (file bytes / parsed text)
- [x] Implement `Change` enum: NEW / UNCHANGED / CHANGED / DELETED
- [x] Implement change detection from current hashes vs. previous catalog
- [x] Enforce idempotency: repeated runs with no document changes produce no new embeddings or snapshots

### Parsing (`src/ingestion/parser.py`)

- [x] Implement PDF parser with `pypdf`
- [x] Implement DOCX parser with `python-docx`
- [x] Implement HTML parser with `beautifulsoup4`
- [x] Implement TXT parser
- [x] Implement database-row parser (rows projected to text documents; MVP: SQLite)
- [x] Return normalized `{document_id, source, format, text, metadata, collected_at}` output
- [x] Handle parse failure: raise + log (skip handled in pipeline)

### Loading (`src/ingestion/loader.py`)

- [x] Implement source profiles (filesystem, web, confluence, sharepoint, database)
- [x] Implement recursive filesystem discovery (PDF/DOCX/TXT)
- [x] Route each discovered document to the matching parser
- [x] Pass through source metadata for lineage capture
- [x] Register connectors as a source registry (filesystem + database wired; confluence/sharepoint/web connector profiles)

### Chunking (`src/chunking/chunker.py`)

- [x] Wrap LlamaIndex `NodeParser` with `CHUNK_SIZE=800` / `CHUNK_OVERLAP=150`
- [x] Generate deterministic `chunk_id` (`{document_id}::{version}::chunk::{index}`)
- [x] Track `block_start` / `block_end` offsets per chunk
- [x] Handle empty text and short documents

### Embeddings (`src/embeddings/embedder.py`)

- [x] Create OpenAI-compatible client from `OPENROUTER_BASE_URL` / `OPENROUTER_API_KEY`
- [x] Register embedding model id and capture embedding dimension (recorded on first embedding)
- [x] Validate dimension consistency (mismatch → log critical, do not add vector, do not persist snapshot)
- [x] Detect embedding-model change → existing snapshot incompatible → full reindex
- [x] Add retry + logging on embedding failure

### Index Build — minimal (`src/indexing/builder.py`)

- [x] Build a snapshot into a versioned Chroma collection via `ChromaVectorStore`
- [x] Persist chunk nodes with provenance metadata
- [x] Return snapshot stats (chunk count, document count, per-source counts)

### CLI — index (`src/main.py`)

- [x] Implement `rag-dataops index`: discover → hash → change-detect → parse → chunk → embed → build snapshot → bump version
- [x] Skip UNCHANGED documents with no embedding requests
- [x] Replace CHANGED documents in the snapshot
- [x] Remove DELETED documents from the snapshot
- [x] Keep going after per-document failures; stop + log critical on snapshot failures (registry not mutated)

## RAGOps 2 — Document Metadata (lineage + provenance)

### Document Catalog (`src/ingestion/` + registry storage)

- [x] Implement `document_catalog.json` persistence (version, hash, last_modified, source, format, chunk_ids)
- [x] Track document versions derived from hash history (e.g. `v4`)
- [x] Record version history so lineage can be traced across index snapshots

### Metadata Extraction & Lineage

- [x] Capture source metadata at ingestion time (author, url, table, row keys, collected_at, ...)
- [x] Stamp every chunk node with provenance fields: document_id, version, source, format, chunk_index, block_start, block_end, index_version, chunk_hash
- [x] Compute deterministic `chunk_hash` for cross-snapshot chunk recognition
- [x] Implement lineage lookup: chunk → document → version → source → index snapshot → embedding model/dimension

### CLI — catalog (`src/main.py`)

- [x] Implement `rag-dataops catalog` showing document versions, hashes, and lineage

## RAGOps 3 — Index Versioning (reproducibility + rollback)

### Index Registry (`src/indexing/registry.py`)

- [x] Implement `index_registry.json` persistence: `current_version` pointer + append-only version records
- [x] Record per version: created_at, embedding_model, embedding_dimension, embedding_provider, chunk_size, chunk_overlap, collection_name, chunk_count, document_count, per-source stats
- [x] Enforce immutability of existing version records (only `current_version` may change)

### Versioning (`src/indexing/versioning.py`)

- [x] Implement snapshot lifecycle: build snapshot → create versioned Chroma collection → write version config → point `current_version`
- [x] Ensure a failed snapshot build does not mutate the registry or `current_version`
- [x] Implement rollback: verify target snapshot exists, repoint `current_version`, validate restored collection
- [x] Fail loudly if the target snapshot is missing on disk

### Retrieval (`src/retrieval/retriever.py`)

- [x] Embed the query with the registered embedding model
- [x] Search the current versioned Chroma collection via LlamaIndex
- [x] Require query embedding model to match the registered snapshot model
- [x] Return top-k chunks with lineage metadata

### CLI — full (`src/main.py`)

- [x] Implement `rag-dataops versions` listing index versions + configuration
- [x] Implement `rag-dataops rollback <version>`
- [x] Implement `rag-dataops search "<query>"` returning chunks with lineage metadata

> **Out of scope for this version:** retrieval metrics (Recall@k, MRR, Precision@k) — deferred to the Roadmap (`docs/PROJECT.md` §21).

## Streamlit Dashboard (`app/`)

- [x] Scaffold the multipage app: `app/app.py` entrypoint + `app/pages/` (Documents, Index Versions, Lineage, Search, Rollback)
- [x] Overview page (`app.py`): current version, chunk/document counts, per-source distribution
- [x] Documents page: catalog entries (version, hash, last_modified, source, format, chunk_ids) rendered as a table with filtering
- [x] Index Versions page: registry versions + configuration (embedding model, dimension, provider, chunk size/overlap, counts)
- [x] Lineage page: drill from chunk → document → version → source connector → index snapshot
- [x] Search page: question input, top-k results with lineage metadata (calls `src/retrieval/retriever.py`)
- [x] Rollback page: version selector, confirmation, calls `src/indexing/versioning.py` rollback (enforces snapshot-exists guardrail)
- [x] Read `indexes/index_registry.json` and `indexes/document_catalog.json` for all read-only pages
- [x] Keep dashboard read-only except the Rollback page; no indexing from the UI

## Logging

- [x] Log to `logs/rag-dataops.log` and console (INFO): discovery, NEW/CHANGED, snapshot build, version registration, rollback
- [x] Log errors: parse failure, embedding failure, dimension mismatch, snapshot build failure, rollback failure

## Tests

- [x] `test_config.py`: settings defaults, env overrides, path anchoring, validation
- [x] `test_hashing.py`: NEW/UNCHANGED/CHANGED/DELETED detection; idempotency of repeat runs; streaming hash determinism
- [x] `test_parser.py`: PDF, DOCX, HTML, TXT, database-row parsing into normalized documents; failure handling
- [x] `test_loader.py`: recursive discovery, connector routing, source metadata passthrough
- [x] `test_chunker.py`: chunk size, overlap, provenance metadata, chunk identity, empty text
- [x] `test_embedder.py`: model registration, dimension capture, dimension validation, model-change detection
- [x] `test_builder.py`: snapshot build into versioned collection, chunk/document counts, per-source stats
- [x] `test_registry.py`: version catalog read/write, immutability of existing version records, `current_version` pointer
- [x] `test_versioning.py`: snapshot isolation, rollback restores correct collection, failed rollout leaves state intact, missing snapshot fails loudly
- [x] `test_retriever.py`: query embedding, model-match enforcement, top-k results with lineage metadata
- [x] `test_main.py`: CLI dispatch (index, versions, rollback, catalog, search); per-document failure continues vs. critical aborts; unchanged-run idempotency; logging setup
- [x] `test_dashboard.py`: pages import and render against sample catalog/registry JSON (smoke test)

## CI/CD & Documentation

- [ ] Verify GitHub Actions workflow passes (UV sync, ruff, pytest)
- [ ] `test_readme.py`: README structure references, CLI subcommands, and CI gate stay in sync with the package
- [x] Refresh README / PROJECT / TODO to match the implemented state at each phase boundary
- [ ] Add `docs/PROJECT.md` markdown linting/consistency check to CI (optional)