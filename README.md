# telecom-rag

Step-by-step RAG over telecom policy documents (billing, roaming, plans).
Built for learning and as a portfolio project.

## Quickstart

    cp .env.example .env
    make install
    make db-up      # Postgres + pgvector in Docker
    make migrate    # create schemas and tables
    make index      # chunk, embed and store the corpus
    make check      # ruff + pytest

## Status

Step 0: project skeleton. See the roadmap in `docs/` (coming in later steps).