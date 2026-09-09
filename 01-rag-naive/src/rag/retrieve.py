"""Semantic retrieval: embed the question, search FAISS, map to metadata."""

from __future__ import annotations

import logging

from pydantic import BaseModel, ConfigDict, Field

from rag.config import Settings
from rag.embed import Embedder
from rag.index import (
    IndexIncompatibleError,
    IndexOperationError,
    ManifestStore,
    MetadataStore,
    VectorIndex,
)

logger = logging.getLogger(__name__)


class RetrievalResult(BaseModel):
    """A retrieved chunk mapped back to its source text and page numbers."""

    model_config = ConfigDict(frozen=True)

    vector_id: int = Field(ge=0)
    score: float
    document_id: str
    chunk_id: str
    source: str
    page_start: int = Field(gt=0)
    page_end: int = Field(gt=0)
    text: str


class Retriever:
    """Embed a question and search the persisted FAISS index."""

    def __init__(self, embedder: Embedder, *, settings: Settings) -> None:
        self.embedder = embedder
        self.settings = settings
        self.metadata_store = MetadataStore(settings.metadata_path)
        self.manifest_store = ManifestStore(settings.manifest_path)
        self.metadata = self.metadata_store.load()
        self.manifest = self.manifest_store.load()

        if self.manifest is None:
            raise IndexOperationError("index_manifest.json not found; run indexing first")
        if self.manifest.embedding_model != self.embedder.model:
            raise IndexIncompatibleError(
                f"index was built with embedding model {self.manifest.embedding_model!r} "
                f"but configured model is {self.embedder.model!r}; run a full reindex"
            )
        if not settings.index_path.is_file():
            raise IndexOperationError("faiss.index not found; run indexing first")
        self.index = VectorIndex.load(settings.index_path)
        if (
            self.manifest.embedding_dimension is not None
            and self.index.dimension != self.manifest.embedding_dimension
        ):
            raise IndexIncompatibleError(
                f"index dimension {self.index.dimension} does not match manifest "
                f"dimension {self.manifest.embedding_dimension}; run a full reindex"
            )

    def search(self, question: str, *, top_k: int | None = None) -> list[RetrievalResult]:
        """Embed ``question``, search the index, and return ranked results."""
        k = self.settings.top_k if top_k is None else top_k
        if k < 1:
            raise ValueError("top_k must be >= 1")

        query = self.embedder.embed_texts(
            [question], expected_dimension=self.manifest.embedding_dimension
        )[0]
        vector_ids, scores = self.index.search(query.vector, k)

        results: list[RetrievalResult] = []
        for vector_id, score in zip(vector_ids, scores, strict=True):
            record = self.metadata.get(vector_id)
            if record is None:
                logger.error("FAISS returned vector id %s with no metadata record", vector_id)
                continue
            results.append(
                RetrievalResult(
                    vector_id=record.vector_id,
                    score=score,
                    document_id=record.document_id,
                    chunk_id=record.chunk_id,
                    source=record.source,
                    page_start=record.page_start,
                    page_end=record.page_end,
                    text=record.text,
                )
            )
        return results
