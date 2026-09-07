"""Unit tests for RequestLogger (src/observability.py). Uses temp dirs, no IO elsewhere."""

from __future__ import annotations

import json
import time
from pathlib import Path

import pytest

from src.observability import DEFAULT_LOG_FILE, RequestLogger, new_request_id

CHUNKS = [
    {"chunk_id": "chunk_003", "source": "example.txt", "score": 0.91, "content": "abc"},
    {"chunk_id": "chunk_008", "source": "other.txt", "score": 0.84, "content": "def"},
]


@pytest.fixture
def log_path(tmp_path):
    return tmp_path / "logs" / "rag_requests.jsonl"


def test_new_request_id_is_unique() -> None:
    ids = {new_request_id() for _ in range(1000)}
    assert len(ids) == 1000


def test_start_records_question(log_path) -> None:
    logger = RequestLogger(log_path)
    logger.start("What is RAG?")
    assert logger.record["question"] == "What is RAG?"
    assert logger.record["request_id"]
    assert "timestamp" in logger.record


def test_set_retrieval_records_chunks_and_scores(log_path) -> None:
    logger = RequestLogger(log_path)
    logger.start("What is RAG?")
    logger.set_retrieval(CHUNKS)
    assert [c["chunk_id"] for c in logger.record["retrieved_chunks"]] == ["chunk_003", "chunk_008"]
    assert [c["score"] for c in logger.record["retrieved_chunks"]] == [0.91, 0.84]
    assert logger.record["retrieval_scores"] == [0.91, 0.84]


def test_set_prompt_and_model(log_path) -> None:
    logger = RequestLogger(log_path)
    logger.start("q").set_prompt("system prompt text").set_model("minimax/MiniMax-M2")
    assert logger.record["prompt"] == "system prompt text"
    assert logger.record["model"] == "minimax/MiniMax-M2"


def test_finish_computes_latency(log_path) -> None:
    logger = RequestLogger(log_path)
    logger.start("q")
    time.sleep(0.01)
    record = logger.finish(answer="ans", usage={"total_tokens": 42}, sources=["a.txt"])
    assert record["answer"] == "ans"
    assert record["sources"] == ["a.txt"]
    assert record["usage"]["total_tokens"] == 42
    assert record["latency"] is not None and record["latency"] > 0


def test_records_error(log_path) -> None:
    logger = RequestLogger(log_path)
    logger.start("q").record_error(ValueError("boom"))
    logger.flush()
    line = log_path.read_text().strip().splitlines()[0]
    record = json.loads(line)
    assert record["error"] == "boom"


def test_flush_writes_jsonl(log_path) -> None:
    logger = RequestLogger(log_path)
    logger.start("What is RAG?")
    logger.set_retrieval(CHUNKS)
    logger.set_prompt("p").set_model("m")
    logger.finish(answer="A RAG is...", usage={"total_tokens": 10}, sources=["example.txt"])
    logger.flush()

    assert log_path.exists()
    with open(log_path, encoding="utf-8") as fh:
        record = json.loads(fh.readline().strip())
    assert record["question"] == "What is RAG?"
    assert "request_id" in record
    assert "answer" in record
    assert record["sources"] == ["example.txt"]
    assert record["usage"] == {"total_tokens": 10}


def test_flush_noop_without_record(log_path) -> None:
    RequestLogger(log_path).flush()
    assert not log_path.exists()


def test_render_matches_documented_format(log_path) -> None:
    logger = RequestLogger(log_path)
    logger.start("What is RAG?")
    logger.set_retrieval(CHUNKS)
    logger.set_prompt("p").set_model("gpt-...")
    record = logger.finish(answer="RAG = retrieval augmented generation", latency=1.43, sources=["example.txt"])
    text = logger.render(record)

    assert "REQUEST" in text
    assert "What is RAG?" in text
    assert "chunk_003  score=0.910" in text
    assert "gpt-..." in text
    assert "Latency:" in text and "1.43s" in text
    assert "RAG = retrieval augmented generation" in text
    assert "example.txt" in text


def test_context_manager_flushes(log_path) -> None:
    with RequestLogger(log_path) as logger:
        logger.start("q")
        logger.finish(answer="a")
    record = json.loads(log_path.read_text().strip().splitlines()[0])
    assert record["answer"] == "a"