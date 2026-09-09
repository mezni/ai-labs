"""FAISS vector indexing, metadata, manifest, and end-to-end document indexing."""

from __future__ import annotations

import json
import logging
import os
from collections.abc import Mapping, Sequence
from pathlib import Path

import faiss
import numpy as np
from pydantic import BaseModel, ConfigDict, Field

from rag.chunk import Chunk, chunk_document
from rag.clean import clean_pages
from rag.config import Settings
from rag.embed import Embedder, Embedding
from rag.extract import ExtractedDocument
from rag.state import DocumentState, DocumentStateStore, atomic_write_json

logger = logging.getLogger(__name__)

INDEX_TYPE = "IndexIDMap2(IndexFlatIP)"
SIMILARITY = "cosine"


class IndexOperationError(RuntimeError):
    """Raised when an index mutation fails."""


class IndexConsistencyError(IndexOperationError):
    """Raised when FAISS IDs, metadata IDs, and document-state IDs disagree."""


class IndexIncompatibleError(IndexOperationError):
    """Raised when a full reindex is required (model/dimension/config change)."""


class MetadataRecord(BaseModel):
    """Per-vector metadata used to map FAISS results back to source text."""

    model_config = ConfigDict(frozen=True)

    vector_id: int = Field(ge=0)
    chunk_id: str
    document_id: str
    source: str
    chunk_index: int = Field(ge=0)
    page_start: int = Field(gt=0)
    page_end: int = Field(gt=0)
    text: str


class IndexManifest(BaseModel):
    """Describes the persisted FAISS index and protects against incompatibilities."""

    model_config = ConfigDict(frozen=True)

    version: int = Field(default=1, ge=1)
    embedding_model: str = ""
    embedding_dimension: int | None = Field(default=None, ge=1)
    similarity: str = SIMILARITY
    index_type: str = INDEX_TYPE
    next_vector_id: int = Field(default=0, ge=0)
    vector_count: int = Field(default=0, ge=0)


class MetadataStore:
    """Persist ``{vector_id: MetadataRecord}`` as JSON keyed by string ID."""

    def __init__(self, path: Path) -> None:
        self.path = path

    def load(self) -> dict[int, MetadataRecord]:
        if not self.path.is_file():
            return {}
        with self.path.open("r", encoding="utf-8") as handle:
            data = json.load(handle)
        return {
            int(vector_id): MetadataRecord.model_validate(record)
            for vector_id, record in data.items()
        }

    def save(self, records: Mapping[int, MetadataRecord]) -> None:
        payload = {
            str(vector_id): record.model_dump(mode="json") for vector_id, record in records.items()
        }
        atomic_write_json(self.path, payload)


class ManifestStore:
    """Persist the :class:`IndexManifest` atomically."""

    def __init__(self, path: Path) -> None:
        self.path = path

    def load(self) -> IndexManifest | None:
        if not self.path.is_file():
            return None
        with self.path.open("r", encoding="utf-8") as handle:
            return IndexManifest.model_validate(json.load(handle))

    def save(self, manifest: IndexManifest) -> None:
        atomic_write_json(self.path, manifest.model_dump(mode="json"))


class VectorIndex:
    """Thin wrapper around FAISS ``IndexIDMap2(IndexFlatIP(dim))``."""

    def __init__(self, index: faiss.IndexIDMap2) -> None:
        self._index = index

    @classmethod
    def create(cls, dimension: int) -> VectorIndex:
        return cls(faiss.IndexIDMap2(faiss.IndexFlatIP(dimension)))

    @classmethod
    def load(cls, path: Path) -> VectorIndex:
        index = faiss.read_index(str(path))
        if not isinstance(index, faiss.IndexIDMap2):
            raise IndexOperationError(f"expected {INDEX_TYPE} index, got {type(index).__name__}")
        return cls(index)

    @property
    def dimension(self) -> int:
        return self._index.d

    @property
    def ntotal(self) -> int:
        return self._index.ntotal

    def ids(self) -> set[int]:
        if self._index.ntotal == 0:
            return set()
        return {int(vector_id) for vector_id in faiss.vector_to_array(self._index.id_map)}

    def _matrix(self, vectors) -> np.ndarray:
        matrix = np.ascontiguousarray(vectors, dtype=np.float32)
        if matrix.ndim != 2:
            raise IndexOperationError(f"vectors must be 2D, got {matrix.ndim}D")
        if matrix.shape[1] != self.dimension:
            raise IndexOperationError(
                f"vector dimension {matrix.shape[1]} does not match "
                f"index dimension {self.dimension}"
            )
        return matrix

    def add(self, vectors, ids: Sequence[int]) -> None:
        matrix = self._matrix(vectors)
        id_array = np.ascontiguousarray(np.asarray(ids, dtype=np.int64))
        if matrix.shape[0] != id_array.shape[0]:
            raise IndexOperationError(f"got {matrix.shape[0]} vectors for {id_array.shape[0]} ids")
        faiss.normalize_L2(matrix)
        self._index.add_with_ids(matrix, id_array)

    def remove(self, ids: Sequence[int]) -> None:
        if not ids:
            return
        id_array = np.ascontiguousarray(np.asarray(ids, dtype=np.int64))
        self._index.remove_ids(id_array)

    def search(self, query_vector, k: int) -> tuple[list[int], list[float]]:
        if k < 1:
            raise IndexOperationError("k must be >= 1")
        query = np.ascontiguousarray(query_vector, dtype=np.float32).reshape(1, -1)
        if query.shape[1] != self.dimension:
            raise IndexOperationError(
                f"query dimension {query.shape[1]} does not match index dimension {self.dimension}"
            )
        faiss.normalize_L2(query)
        scores, ids = self._index.search(query, k)
        found_ids: list[int] = []
        found_scores: list[float] = []
        for vector_id, score in zip(ids[0], scores[0], strict=True):
            if vector_id == -1:
                break
            found_ids.append(int(vector_id))
            found_scores.append(float(score))
        return found_ids, found_scores

    def save(self, path: Path) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        tmp_path = path.with_name(f"{path.name}.tmp")
        faiss.write_index(self._index, str(tmp_path))
        os.replace(tmp_path, path)


def validate_consistency(
    index: VectorIndex,
    metadata: Mapping[int, MetadataRecord],
    state: Mapping[str, DocumentState],
    manifest: IndexManifest,
) -> None:
    """Fail loudly unless FAISS, metadata, and document state agree."""
    problems: list[str] = []
    index_ids = index.ids()
    metadata_ids = set(metadata)
    state_ids = set()

    if index.ntotal != len(metadata):
        problems.append(
            f"FAISS has {index.ntotal} vectors but metadata has {len(metadata)} records"
        )
    if index.ntotal != manifest.vector_count:
        problems.append(
            f"FAISS has {index.ntotal} vectors but manifest records {manifest.vector_count}"
        )
    if index_ids != metadata_ids:
        differing = len(index_ids ^ metadata_ids)
        problems.append(f"FAISS id set does not match metadata ids ({differing} differ)")

    for document_id, document_state in state.items():
        if document_state.chunk_count != len(document_state.vector_ids):
            problems.append(
                f"{document_id} state records {document_state.chunk_count} chunks "
                f"but {len(document_state.vector_ids)} vector ids"
            )
        for vector_id in document_state.vector_ids:
            if vector_id in state_ids:
                problems.append(f"vector id {vector_id} owned by multiple documents")
            state_ids.add(vector_id)
            if vector_id not in index_ids:
                problems.append(f"state vector id {vector_id} missing from FAISS")
            if vector_id not in metadata_ids:
                problems.append(f"state vector id {vector_id} missing from metadata")

    for vector_id, record in metadata.items():
        document_state = state.get(record.document_id)
        if document_state is None:
            problems.append(
                f"metadata vector id {vector_id} references missing document {record.document_id!r}"
            )
        elif vector_id not in document_state.vector_ids:
            problems.append(
                f"metadata vector id {vector_id} not listed in "
                f"{record.document_id!r} document state"
            )

    if index_ids != state_ids:
        problems.append("FAISS id set does not match document-state vector ids")

    if problems:
        raise IndexConsistencyError("; ".join(problems))


class Indexer:
    """End-to-end indexing: chunk → embed → validate → add vectors → persist."""

    def __init__(self, embedder: Embedder, *, settings: Settings) -> None:
        self.embedder = embedder
        self.settings = settings
        self.document_state_store = DocumentStateStore(settings.document_state_path)
        self.metadata_store = MetadataStore(settings.metadata_path)
        self.manifest_store = ManifestStore(settings.manifest_path)

        self.state = self.document_state_store.load()
        self.metadata_records: dict[int, MetadataRecord] = self.metadata_store.load()
        self.manifest = self.manifest_store.load()
        self.index: VectorIndex | None

        persisted = [
            path
            for path in (
                settings.index_path,
                settings.metadata_path,
                settings.document_state_path,
                settings.manifest_path,
            )
            if path.exists()
        ]
        if persisted and self.manifest is None:
            raise IndexOperationError(
                "persisted index files exist but index_manifest.json is missing; run a full reindex"
            )

        self._check_model_compatibility()

        if self.manifest is None:
            self.manifest = IndexManifest(embedding_model=self.embedder.model)
            self.index = None
            return

        if not settings.index_path.is_file():
            raise IndexOperationError(
                "index_manifest.json exists but faiss.index is missing; run a full reindex"
            )
        self.index = VectorIndex.load(settings.index_path)
        if (
            self.manifest.embedding_dimension is not None
            and self.index.dimension != self.manifest.embedding_dimension
        ):
            raise IndexIncompatibleError(
                f"index dimension {self.index.dimension} does not match manifest "
                f"dimension {self.manifest.embedding_dimension}; run a full reindex"
            )
        validate_consistency(self.index, self.metadata_records, self.state, self.manifest)

    def _check_model_compatibility(self) -> None:
        if self.manifest is not None and self.manifest.embedding_model != self.embedder.model:
            raise IndexIncompatibleError(
                f"index was built with embedding model {self.manifest.embedding_model!r} "
                f"but configured model is {self.embedder.model!r}; run a full reindex"
            )

    def index_document(self, document: ExtractedDocument, document_hash: str) -> list[int]:
        """Index a new document version, or safely replace it if already present."""
        if document.document_id in self.state:
            return self.replace_document(document, document_hash)
        return self._add_document(document, document_hash)

    def replace_document(self, document: ExtractedDocument, document_hash: str) -> list[int]:
        """Replace an indexed document without losing its last known-good version.

        The new version is embedded, validated, persisted, and only then are the
        old vectors and metadata removed.
        """
        previous = self.state.get(document.document_id)
        if previous is None:
            return self._add_document(document, document_hash)

        chunks, embeddings = self._prepare(document)
        new_ids = self._allocate(len(chunks))
        self._add_vectors(document.document_id, chunks, embeddings, new_ids)
        self.state[document.document_id] = DocumentState(
            hash=document_hash, vector_ids=new_ids, chunk_count=len(chunks)
        )
        self._persist()

        old_ids = list(previous.vector_ids)
        if old_ids:
            self.index.remove(old_ids)
            for vector_id in old_ids:
                self.metadata_records.pop(vector_id, None)
            self._persist()

        self._validate_invariants()
        return new_ids

    def remove_document(self, document_id: str) -> None:
        """Delete a document's vectors, metadata, and state."""
        previous = self.state.pop(document_id, None)
        if previous is None:
            logger.warning("remove_document called for unknown document %s", document_id)
            return
        if previous.vector_ids:
            self.index.remove(previous.vector_ids)
            for vector_id in previous.vector_ids:
                self.metadata_records.pop(vector_id, None)
        self._persist()
        self._validate_invariants()

    def _add_document(self, document: ExtractedDocument, document_hash: str) -> list[int]:
        chunks, embeddings = self._prepare(document)
        vector_ids = self._allocate(len(chunks))
        self._add_vectors(document.document_id, chunks, embeddings, vector_ids)
        self.state[document.document_id] = DocumentState(
            hash=document_hash, vector_ids=vector_ids, chunk_count=len(chunks)
        )
        self._persist()
        self._validate_invariants()
        return vector_ids

    def _prepare(self, document: ExtractedDocument) -> tuple[list[Chunk], list[Embedding]]:
        cleaned = ExtractedDocument(
            document_id=document.document_id, pages=clean_pages(document.pages)
        )
        chunks = chunk_document(
            cleaned,
            chunk_size=self.settings.chunk_size,
            overlap=self.settings.chunk_overlap,
        )
        if not chunks:
            raise IndexOperationError(f"document {document.document_id!r} produced no chunks")

        embeddings = self.embedder.embed_texts(
            [chunk.text for chunk in chunks], expected_dimension=self.manifest.embedding_dimension
        )
        if len(embeddings) != len(chunks):
            raise IndexOperationError(
                f"embedded {len(embeddings)} vectors for {len(chunks)} chunks"
            )
        embeddings.sort(key=lambda embedding: embedding.index)
        self._adopt_dimension(len(embeddings[0].vector))
        return chunks, embeddings

    def _adopt_dimension(self, dimension: int) -> None:
        if self.manifest.embedding_dimension is None:
            self.manifest = self.manifest.model_copy(update={"embedding_dimension": dimension})
            self.index = VectorIndex.create(dimension)
        elif self.manifest.embedding_dimension != dimension:
            raise IndexIncompatibleError(
                f"embedding dimension {dimension} does not match manifest dimension "
                f"{self.manifest.embedding_dimension}; run a full reindex"
            )
        elif self.index is None:
            self.index = VectorIndex.create(dimension)

    def _add_vectors(
        self,
        document_id: str,
        chunks: list[Chunk],
        embeddings: list[Embedding],
        vector_ids: Sequence[int],
    ) -> None:
        vectors = np.stack(
            [np.asarray(embedding.vector, dtype=np.float32) for embedding in embeddings]
        )
        records: dict[int, MetadataRecord] = {}
        for chunk, vector_id in zip(chunks, vector_ids, strict=True):
            records[vector_id] = MetadataRecord(
                vector_id=vector_id,
                chunk_id=chunk.chunk_id,
                document_id=document_id,
                source=document_id,
                chunk_index=chunk.chunk_index,
                page_start=chunk.page_start,
                page_end=chunk.page_end,
                text=chunk.text,
            )
        self.index.add(vectors, vector_ids)
        self.metadata_records.update(records)

    def _allocate(self, count: int) -> list[int]:
        start = self.manifest.next_vector_id
        vector_ids = list(range(start, start + count))
        self.manifest = self.manifest.model_copy(update={"next_vector_id": start + count})
        return vector_ids

    def _persist(self) -> None:
        if self.index is None:
            raise IndexOperationError("no FAISS index to persist")
        self.manifest = self.manifest.model_copy(update={"vector_count": self.index.ntotal})
        self.index.save(self.settings.index_path)
        self.metadata_store.save(self.metadata_records)
        self.document_state_store.save(self.state)
        self.manifest_store.save(self.manifest)

    def _validate_invariants(self) -> None:
        if self.index is None:
            raise IndexOperationError("no FAISS index to validate")
        validate_consistency(self.index, self.metadata_records, self.state, self.manifest)
