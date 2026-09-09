from __future__ import annotations

import hashlib
import re

from llama_index.core.node_parser import SentenceSplitter
from llama_index.core.schema import TextNode

from src.ingestion.parser import ParsedDocument

_SENTENCE_SPLIT = re.compile(r"(?<=[.!?])\s+|\n+")


def _count_tokens(text: str) -> list[str]:
    """nltk-free tokenizer: SentenceSplitter only needs token counts."""
    return re.findall(r"\S+", text)


def _split_sentences(text: str) -> list[str]:
    return [part for part in _SENTENCE_SPLIT.split(text) if part.strip()]


def chunk_hash(text: str) -> str:
    """Deterministic hash of chunk text for cross-snapshot recognition."""
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


class Chunker:
    """LlamaIndex NodeParser wrapper that stamps provenance metadata.

    Produces ``chunk_id`` of the form ``{document_id}::{version}::chunk::{index}``
    (PROJECT.md §9) and records character offsets per chunk.
    """

    def __init__(self, chunk_size: int = 800, chunk_overlap: int = 150):
        self.chunk_size = chunk_size
        self.chunk_overlap = chunk_overlap
        self._splitter = SentenceSplitter(
            chunk_size=chunk_size,
            chunk_overlap=chunk_overlap,
            tokenizer=_count_tokens,
            chunking_tokenizer_fn=_split_sentences,
        )

    def chunk(self, document: ParsedDocument, version: str) -> list[TextNode]:
        text = document.text.strip()
        if not text:
            return []

        texts = self._splitter.split_text(text)
        start = 0
        nodes: list[TextNode] = []
        for index, chunk_text in enumerate(texts):
            chunk_text = chunk_text.strip()
            if not chunk_text:
                continue
            end = min(len(text), start + len(chunk_text))
            chunk_id = (
                f"{document.document_id}::{version}::chunk::{index}"
            )
            nodes.append(
                TextNode(
                    text=chunk_text,
                    id_=chunk_id,
                    metadata={
                        "document_id": document.document_id,
                        "version": version,
                        "source": document.source,
                        "format": document.format,
                        "chunk_index": index,
                        "block_start": start,
                        "block_end": end,
                        "chunk_hash": chunk_hash(chunk_text),
                    },
                )
            )
            # advance by chunk length, stepping back by the overlap to keep
            # block_start/block_end aligned with the splitter's sliding window
            start = end - self.chunk_overlap if index < len(texts) - 1 else end
        return nodes