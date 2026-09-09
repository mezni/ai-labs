from __future__ import annotations

import pytest

from src.chunking.chunker import Chunker, chunk_hash
from src.ingestion.parser import ParsedDocument


def make_document(text: str, document_id: str = "a.pdf") -> ParsedDocument:
    return ParsedDocument(
        document_id=document_id,
        source="filesystem",
        format="pdf",
        text=text,
        metadata={},
        collected_at="2026-09-01T00:00:00Z",
    )


def test_empty_text_produces_no_chunks():
    assert Chunker(100, 20).chunk(make_document("   "), "v1") == []


def test_short_document_is_a_single_chunk():
    nodes = Chunker(800, 150).chunk(make_document("Short text."), "v1")
    assert len(nodes) == 1
    assert nodes[0].text.strip() == "Short text."


def test_long_document_splits_with_max_chunk_size():
    text = " ".join(["paragraph"] * 400)  # comfortably over 200 tokens
    chunker = Chunker(200, 20)
    nodes = chunker.chunk(make_document(text), "v1")
    assert len(nodes) > 1
    assert all(len(node.text) < len(text) for node in nodes)


def test_chunk_id_embeds_document_id_and_version():
    nodes = Chunker(50, 10).chunk(make_document("AAA " * 200, "refund-policy.pdf"), "v4")
    assert nodes[0].id_ == "refund-policy.pdf::v4::chunk::0"
    assert nodes[1].id_ == "refund-policy.pdf::v4::chunk::1"


def test_provenance_metadata_present():
    nodes = Chunker(100, 10).chunk(make_document("B " * 120, "handbook.docx"), "v2")
    metadata = nodes[0].metadata
    assert metadata["document_id"] == "handbook.docx"
    assert metadata["version"] == "v2"
    assert metadata["source"] == "filesystem"
    assert metadata["format"] == "pdf"
    assert metadata["chunk_index"] == 0
    assert metadata["block_start"] >= 0
    assert metadata["block_end"] > metadata["block_start"]
    assert metadata["chunk_hash"] == chunk_hash(nodes[0].text)


def test_block_offsets_advance_across_chunks():
    text = "Z " * 300
    chunker = Chunker(100, 10)
    nodes = chunker.chunk(make_document(text), "v1")
    starts = [node.metadata["block_start"] for node in nodes]
    ends = [node.metadata["block_end"] for node in nodes]
    assert starts == sorted(starts)
    assert ends == sorted(ends)
    assert all(end > start for start, end in zip(starts, ends))


def test_chunk_hash_deterministic():
    assert chunk_hash("same text") == chunk_hash("same text")
    assert chunk_hash("same text") != chunk_hash("different text")


def test_document_version_from_parser_contents_used_in_ids():
    nodes = Chunker(60, 10).chunk(make_document("X " * 40, "b.txt"), "v7")
    assert nodes[0].id_.startswith("b.txt::v7::chunk::")
    assert all("::v7::chunk::" in node.id_ for node in nodes)