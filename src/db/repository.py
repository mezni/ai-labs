from collections.abc import Sequence
from itertools import batched

from sqlalchemy.dialects.postgresql import insert
from sqlalchemy.orm import Session

from core.models import Chunk
from db.models import ChunkEmbeddingRow, ChunkRow

_CHUNK_COLUMNS = (
    "doc_id",
    "version",
    "domain",
    "ordinal",
    "text",
    "heading_path",
    "clause",
    "page",
    "token_count",
)


def upsert_chunks(session: Session, chunks: Sequence[Chunk]) -> None:
    for group in batched(chunks, 500):
        rows = [
            {"chunk_id": c.chunk_id, **c.model_dump(include=set(_CHUNK_COLUMNS))} for c in group
        ]
        stmt = insert(ChunkRow).values(rows)
        stmt = stmt.on_conflict_do_update(
            index_elements=["chunk_id"],
            set_={column: stmt.excluded[column] for column in _CHUNK_COLUMNS},
        )
        session.execute(stmt)


def upsert_embeddings(
    session: Session,
    *,
    chunk_ids: Sequence[str],
    vectors: Sequence[list[float]],
    provider: str,
    model: str,
) -> None:
    rows = [
        {"chunk_id": chunk_id, "model": model, "provider": provider, "embedding": vector}
        for chunk_id, vector in zip(chunk_ids, vectors, strict=True)
    ]
    for group in batched(rows, 200):
        stmt = insert(ChunkEmbeddingRow).values(list(group))
        stmt = stmt.on_conflict_do_update(
            index_elements=["chunk_id", "model"],
            set_={"provider": stmt.excluded.provider, "embedding": stmt.excluded.embedding},
        )
        session.execute(stmt)
