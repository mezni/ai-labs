"""create app and vectors schemas, chunks and chunk_embeddings

Revision ID: 0001
Revises:
"""

import sqlalchemy as sa
from alembic import op
from pgvector.sqlalchemy import Vector
from sqlalchemy.dialects import postgresql

revision = "0001"
down_revision = None
branch_labels = None
depends_on = None

EMBEDDING_DIM = 1536


def upgrade() -> None:
    op.execute("CREATE EXTENSION IF NOT EXISTS vector")
    op.execute("CREATE SCHEMA IF NOT EXISTS app")
    op.execute("CREATE SCHEMA IF NOT EXISTS vectors")

    op.create_table(
        "chunks",
        sa.Column("chunk_id", sa.String, primary_key=True),
        sa.Column("doc_id", sa.String, nullable=False),
        sa.Column("version", sa.Integer, nullable=False),
        sa.Column("domain", sa.String, nullable=False),
        sa.Column("ordinal", sa.Integer, nullable=False),
        sa.Column("text", sa.Text, nullable=False),
        sa.Column("heading_path", postgresql.ARRAY(sa.Text), nullable=False),
        sa.Column("clause", sa.String),
        sa.Column("page", sa.Integer),
        sa.Column("token_count", sa.Integer, nullable=False),
        sa.Column(
            "created_at", sa.DateTime(timezone=True), server_default=sa.func.now(), nullable=False
        ),
        schema="app",
    )
    op.create_index("ix_chunks_doc_version", "chunks", ["doc_id", "version"], schema="app")

    op.create_table(
        "chunk_embeddings",
        sa.Column("chunk_id", sa.String, nullable=False),
        sa.Column("model", sa.String, nullable=False),
        sa.Column("provider", sa.String, nullable=False),
        sa.Column("embedding", Vector(EMBEDDING_DIM), nullable=False),
        sa.Column(
            "created_at", sa.DateTime(timezone=True), server_default=sa.func.now(), nullable=False
        ),
        sa.PrimaryKeyConstraint("chunk_id", "model"),
        sa.ForeignKeyConstraint(["chunk_id"], ["app.chunks.chunk_id"], ondelete="CASCADE"),
        schema="vectors",
    )
    op.execute(
        "CREATE INDEX ix_chunk_embeddings_hnsw ON vectors.chunk_embeddings "
        "USING hnsw (embedding vector_cosine_ops)"
    )


def downgrade() -> None:
    op.drop_table("chunk_embeddings", schema="vectors")
    op.drop_table("chunks", schema="app")
    op.execute("DROP SCHEMA IF EXISTS vectors")
    op.execute("DROP SCHEMA IF EXISTS app")
