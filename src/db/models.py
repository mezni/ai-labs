from datetime import datetime

from pgvector.sqlalchemy import Vector
from sqlalchemy import DateTime, ForeignKey, Index, Integer, String, Text, func
from sqlalchemy.dialects.postgresql import ARRAY
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column

EMBEDDING_DIM = 1536


class Base(DeclarativeBase):
    pass


class ChunkRow(Base):
    __tablename__ = "chunks"
    __table_args__ = (Index("ix_chunks_doc_version", "doc_id", "version"), {"schema": "app"})

    chunk_id: Mapped[str] = mapped_column(String, primary_key=True)
    doc_id: Mapped[str] = mapped_column(String)
    version: Mapped[int] = mapped_column(Integer)
    domain: Mapped[str] = mapped_column(String)
    ordinal: Mapped[int] = mapped_column(Integer)
    text: Mapped[str] = mapped_column(Text)
    heading_path: Mapped[list[str]] = mapped_column(ARRAY(Text))
    clause: Mapped[str | None] = mapped_column(String)
    page: Mapped[int | None] = mapped_column(Integer)
    token_count: Mapped[int] = mapped_column(Integer)
    created_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), server_default=func.now())


class ChunkEmbeddingRow(Base):
    __tablename__ = "chunk_embeddings"
    __table_args__ = {"schema": "vectors"}

    chunk_id: Mapped[str] = mapped_column(
        ForeignKey("app.chunks.chunk_id", ondelete="CASCADE"), primary_key=True
    )
    model: Mapped[str] = mapped_column(String, primary_key=True)
    provider: Mapped[str] = mapped_column(String)
    embedding: Mapped[list[float]] = mapped_column(Vector(EMBEDDING_DIM))
    created_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), server_default=func.now())
