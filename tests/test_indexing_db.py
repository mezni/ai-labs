from pathlib import Path

import pytest
from sqlalchemy import Engine, func, select
from sqlalchemy.orm import Session

from corpus import chunk_document, load_manifest
from db.models import ChunkEmbeddingRow, ChunkRow
from embeddings.hashing import HashingEmbedder
from indexing import index_corpus

pytestmark = pytest.mark.integration

DATA_DIR = Path(__file__).resolve().parent.parent / "data"


def counts(engine: Engine) -> tuple[int, int]:
    with Session(engine) as session:
        chunks = session.scalar(select(func.count()).select_from(ChunkRow))
        embeddings = session.scalar(select(func.count()).select_from(ChunkEmbeddingRow))
    return chunks or 0, embeddings or 0


def test_index_corpus_stores_chunks_and_embeddings(clean_db: Engine) -> None:
    report = index_corpus(clean_db, DATA_DIR, HashingEmbedder(), batch_size=4)
    assert report.documents == 5
    assert report.chunks > 5
    assert counts(clean_db) == (report.chunks, report.chunks)


def test_index_is_idempotent(clean_db: Engine) -> None:
    first = index_corpus(clean_db, DATA_DIR, HashingEmbedder())
    second = index_corpus(clean_db, DATA_DIR, HashingEmbedder())
    assert first == second
    assert counts(clean_db) == (first.chunks, first.chunks)


def test_chunk_is_its_own_nearest_neighbour(clean_db: Engine) -> None:
    provider = HashingEmbedder()
    index_corpus(clean_db, DATA_DIR, provider)

    entry = next(e for e in load_manifest(DATA_DIR) if e.doc_id == "AW-ROA-002" and e.version == 2)
    target = next(c for c in chunk_document(DATA_DIR, entry) if "$8 per GB" in c.text)
    query = provider.embed([target.text_with_context()])[0]

    with Session(clean_db) as session:
        nearest = session.scalar(
            select(ChunkEmbeddingRow.chunk_id)
            .where(ChunkEmbeddingRow.model == provider.model)
            .order_by(ChunkEmbeddingRow.embedding.cosine_distance(query))
            .limit(1)
        )
    assert nearest == target.chunk_id
