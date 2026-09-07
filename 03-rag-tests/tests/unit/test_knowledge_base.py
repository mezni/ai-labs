"""Unit tests for the FAISS-backed KnowledgeBase."""

from __future__ import annotations

import pytest

from src.knowledge.knowledge_base import KnowledgeBase

LEAVE_DOC = {
    "document_id": "p1",
    "source": "hr/leave.md",
    "content": "annual leave policy allows thirty days " * 40,
}
ACCESS_DOC = {
    "document_id": "p2",
    "source": "it/access.md",
    "content": "access control requires manager approval " * 40,
}


def _kb(fake_embedder, docs=None) -> KnowledgeBase:
    kb = KnowledgeBase(embedder=fake_embedder)
    kb.build_index(docs or [LEAVE_DOC, ACCESS_DOC])
    return kb


def test_build_index_chunks_and_builds(fake_embedder):
    kb = _kb(fake_embedder)
    assert kb.is_built()
    assert len(kb) == 4  # two long docs, chunked in two each
    assert len(kb.chunks) == 4
    kb.close()


def test_search_returns_relevant_document_first(fake_embedder):
    kb = _kb(fake_embedder)
    results = kb.search("how many leave days", top_k=2)
    assert results[0]["document_id"] == "p1"
    assert results[0]["source"] == "hr/leave.md"
    assert results[0]["score"] >= results[1]["score"]
    kb.close()


def test_search_orders_documents_by_similarity(fake_embedder):
    kb = _kb(fake_embedder)
    results = kb.search("who needs access approval", top_k=3)
    # Query maps to the "access" vector, so IT access chunks rank first.
    assert results[0]["document_id"] == "p2"
    leave_ranks = [i for i, c in enumerate(results) if c["document_id"] == "p1"]
    assert leave_ranks, "p1 chunks should appear in results"
    kb.close()


def test_search_assigns_scores(fake_embedder):
    kb = _kb(fake_embedder)
    results = kb.search("access rights", top_k=1)
    assert results[0]["document_id"] == "p2"
    assert results[0]["score"] >= 0
    kb.close()


def test_search_empty_kb_raises(fake_embedder):
    kb = KnowledgeBase(embedder=fake_embedder)
    with pytest.raises(RuntimeError, match="build_index"):
        kb.search("anything")
    kb.close()


def test_build_index_skips_empty_documents(fake_embedder):
    kb = KnowledgeBase(embedder=fake_embedder)
    kb.build_index(
        [
            LEAVE_DOC,
            {"document_id": "empty", "source": "x.md", "content": "   \n  "},
        ]
    )
    assert len(kb.chunks) == 2
    assert all(c["document_id"] == "p1" for c in kb.chunks)
    kb.close()


def test_chunk_metadata_and_overlap(fake_embedder):
    kb = KnowledgeBase(embedder=fake_embedder, chunk_size=40, overlap=10)
    kb.build_index([LEAVE_DOC])
    chunks = kb.chunks
    assert len(chunks) >= 2
    assert chunks[0]["chunk_id"] == "p1_chunk_001"
    assert chunks[1]["chunk_id"] == "p1_chunk_002"
    assert chunks[0]["category"] == "hr"
    assert chunks[1]["source"] == "hr/leave.md"
    assert chunks[0]["content"][-10:] in chunks[1]["content"]
    kb.close()


def test_save_and_load_roundtrip(fake_embedder, tmp_path):
    kb = _kb(fake_embedder)
    index = tmp_path / "idx.bin"
    chunks = tmp_path / "chunks.json"
    kb.save_index(index_path=index, chunks_path=chunks)
    kb.close()

    restored = KnowledgeBase(embedder=fake_embedder, index_path=index, chunks_path=chunks)
    restored.load_index(index_path=index, chunks_path=chunks)
    assert restored.is_built()
    assert len(restored) == 4
    results = restored.search("leave policy", top_k=1)
    assert results[0]["document_id"] == "p1"
    restored.close()


def test_load_index_missing_files_raises(fake_embedder, tmp_path):
    kb = KnowledgeBase(embedder=fake_embedder)
    try:
        kb.load_index(index_path=tmp_path / "missing.bin", chunks_path=tmp_path / "nope.json")
    except FileNotFoundError:
        pass
    else:
        raise AssertionError("expected FileNotFoundError")
    kb.close()


def test_reset_index(fake_embedder):
    kb = _kb(fake_embedder)
    kb.reset_index()
    assert not kb.is_built()
    assert len(kb) == 0
    assert kb.chunks == []
    kb.close()


def test_load_documents_reads_markdown_corpus(fake_embedder, tmp_path):
    corpus = tmp_path / "policies"
    (corpus / "hr").mkdir(parents=True)
    (corpus / "hr" / "leave.md").write_text("leave text here", encoding="utf-8")
    (corpus / "hr" / "readme.txt").write_text("ignored", encoding="utf-8")
    (corpus / "hr" / "blank.md").write_text("   ", encoding="utf-8")

    kb = KnowledgeBase(embedder=fake_embedder, corpus_dir=corpus)
    docs = kb.load_documents()
    assert len(docs) == 1
    assert docs[0]["document_id"] == "leave"
    assert docs[0]["source"] == "hr/leave.md"
    assert docs[0]["content"] == "leave text here"
    kb.close()


def test_search_requires_built_index(fake_embedder):
    kb = KnowledgeBase(embedder=fake_embedder)
    with pytest.raises(RuntimeError, match="build_index"):
        kb.search("anything")
    kb.close()


def pytest_raises_sentinel():
    """Placeholder removed below."""
    raise AssertionError("placeholder")