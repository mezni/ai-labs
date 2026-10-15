# Telecom RAG Development Log

## Step 0: Init project

- **Goal:** A reproducible repo skeleton that anyone can clone and run.
- **Problem:** No structure, no tooling, hard for others to run.
- **Solution (files):** `pyproject.toml`, `.python-version`, `src/`, `tests/test_smoke.py`,
  `.env.example`, `.gitignore`, `README.md` stub, `.github/workflows/ci.yml`, ruff + pytest config.
- **Trade-off:** uv vs poetry: uv is faster and simpler, with one lockfile and no packaging
  ceremony. Code sits directly under `src/` with no intermediate package, so I set
  `package = false` and `pythonpath = ["src"]` instead of building an installable package.
  You give up `pip install .` for now.
- **Example:** `uv run pytest` → 1 passed.

## Step 1: Corpus and domain scope

- **Goal:** A small, realistic telecom document set plus a manifest that describes every document.
- **Problem:** Without realistic docs, retrieval looks fake in a portfolio. Without metadata
  (version, plan type, effective dates), you can't filter stale answers later (step 7).
- **Solution (files):** `data/raw/{billing,roaming,plans,network}/*.md`, `data/manifest.json`,
  `data/README.md`, `tests/test_corpus.py`, one-line fix in `.gitignore`.
- **Trade-off:** Synthetic docs (safe, controllable, no licensing) vs real operator docs
  (realistic, legal risk). Start with 5 short docs instead of 12: enough to test every later
  step, and you can grow the set. Markdown first (diffable in git). PDFs come in step 2 to
  exercise a second parser.
- **Example:** `make test` → 5 passed.

## Step 2: Parse and clean

- **Goal:** Turn raw files (Markdown, text, PDF) into clean, structured blocks that keep their
  lineage (heading level, page).
- **Problem:** Raw parsing leaves page numbers, repeated headers and footers, odd whitespace and
  ligatures. Without structure, chunking (step 3) can't cut on clause boundaries or cite a source.
- **Solution (files):** `src/core/models.py` (`Block`), `src/parsing/` (`_blocks.py`,
  `markdown.py`, `text.py`, `pdf.py`, `registry.py`), `src/cleaning.py`, `tests/test_parsing.py`,
  `tests/test_cleaning.py`, `tests/test_pdf.py`.
- **Trade-off:** Parsers take bytes, not a path, so they stay independent of where files come
  from (filesystem, API, database later). A new block starts at a heading, a clause number
  (`2.1 `) or after a line ending in `.`, so clauses stay separate but a wrapped line that ends
  in a period will be split early. Cleaning drops any text found on 2 or more pages (headers,
  footers), which would also drop a genuine sentence repeated on two pages. pypdf is light and
  has no OCR, so scanned PDFs are out of scope. Heavier parsers like docling can replace it later
  behind the same registry.
- **Example:** `AW-ROA-002_roaming_policy_v2.md` → `heading(level=1)` "AW-ROA-002 Roaming Policy
  (v2)", `heading(level=2)` "2. Zones", `paragraph` "2.1 Zone A covers …".

## Step 3: Chunking with lineage

- **Goal:** Turn parsed blocks into retrievable `Chunk` objects that know where they came from
  (document, version, heading path, clause, page).
- **Problem:** Fixed-size splits cut through clauses and lose their source. A chunk like "billed
  at $8 per GB" is useless if it doesn't say which policy, which version and which section it
  belongs to.
- **Solution (files):** `src/core/models.py` (add `Chunk`), `src/chunking.py`, `src/corpus.py`
  (manifest loader plus parse → clean → chunk glue), `tests/test_chunking.py`,
  `tests/test_chunking_corpus.py`.
- **Trade-off:** Chunks follow clause boundaries instead of a fixed window. Clauses already stand
  alone, so there is no overlap, which keeps the index smaller. Clauses under `min_tokens` merge
  with the next one, so tiny fragments don't become their own vectors. A clause over `max_tokens`
  is split on sentences, then on words, and every piece keeps the same heading path and clause.
  Token counts use a `chars / 4` estimate, so there is no tokenizer dependency yet. The embedding
  provider is configurable, so exact counts are better added in step 4.
- **Example:** Roaming v2, clause 2.2 → `Chunk(chunk_id="AW-ROA-002-v2-0002",
  heading_path=("AW-ROA-002 Roaming Policy (v2)", "2. Zones"), clause="2.2", text="2.2 Zone B
  covers … $8 per GB.")`.

## Step 4: Embeddings and vector store

- **Goal:** Make chunks searchable by meaning: a configurable embedding provider, a Postgres +
  pgvector store, and the first Alembic migration.
- **Problem:** No semantic search yet, and ad hoc vector storage is brittle. Switching providers
  later would touch every caller, and tests would need an API key.
- **Solution (files):** `src/config.py`, `src/embeddings/` (`base.py`, `hashing.py`,
  `openai_provider.py`, `registry.py`), `src/db/` (`models.py`, `repository.py`),
  `src/indexing.py`, `alembic.ini`, `migrations/`, `docker-compose.yml`, `docker/initdb/`,
  updated `Makefile`, `.env.example`, `ci.yml`, `tests`.
- **Trade-off:** The provider sits behind a small protocol chosen by `EMBEDDING_PROVIDER`. The
  hashing provider runs offline, so tests and CI need no API key, but its similarity is lexical,
  not semantic. Real quality numbers come with OpenAI in step 10. The dimension is fixed at 1536
  in the migration, so a model with other dimensions needs a new migration. Two schemas split the
  data: `app` holds chunk text and lineage, `vectors` holds embeddings. No `documents` or
  `pipeline_runs` tables yet (step 11). HNSW is approximate but fast and needs no training, while
  an exact scan would be fine at this size. The whole run is one transaction, to be replaced by
  one per document in step 11. Every run re-embeds everything, which costs API calls until step 11
  adds file hashing.
- **Example:** `make index` → indexed 5 documents, N chunks, N embeddings (hashing-v1, 1536 dims).
