from dataclasses import dataclass
from pathlib import Path

from sqlalchemy import Engine, create_engine
from sqlalchemy.orm import Session

from config import get_settings
from corpus import chunk_document, load_manifest
from db.models import EMBEDDING_DIM
from db.repository import upsert_chunks, upsert_embeddings
from embeddings.base import EmbeddingProvider, embed_all
from embeddings.registry import get_provider


@dataclass(frozen=True)
class IndexReport:
    documents: int
    chunks: int
    embeddings: int


def index_corpus(
    engine: Engine, data_dir: Path, provider: EmbeddingProvider, *, batch_size: int = 64
) -> IndexReport:
    """Chunk every manifest document, embed the chunks and upsert both into Postgres."""
    if provider.dim != EMBEDDING_DIM:
        raise ValueError(f"provider dim {provider.dim} != schema dim {EMBEDDING_DIM}")

    manifest = load_manifest(data_dir)
    chunks = [chunk for entry in manifest for chunk in chunk_document(data_dir, entry)]
    # Embed before opening the transaction so it is not held open during API calls.
    vectors = embed_all(provider, [c.text_with_context() for c in chunks], batch_size)

    with Session(engine) as session, session.begin():
        upsert_chunks(session, chunks)
        upsert_embeddings(
            session,
            chunk_ids=[c.chunk_id for c in chunks],
            vectors=vectors,
            provider=provider.name,
            model=provider.model,
        )
    return IndexReport(documents=len(manifest), chunks=len(chunks), embeddings=len(vectors))


def main() -> None:
    settings = get_settings()
    provider = get_provider(settings)
    engine = create_engine(settings.database_url, pool_pre_ping=True)
    report = index_corpus(engine, Path("data"), provider, batch_size=settings.embedding_batch_size)
    print(
        f"indexed {report.documents} documents, {report.chunks} chunks, "
        f"{report.embeddings} embeddings ({provider.model}, {provider.dim} dims)"
    )


if __name__ == "__main__":
    main()
