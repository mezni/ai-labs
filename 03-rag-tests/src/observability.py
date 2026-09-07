"""Basic observability: per-request telemetry for the RAG pipeline.

Each request is recorded with a stable request_id and a structured JSON log
line capturing the question, retrieved chunks and scores, the prompt, model,
latency, token usage, the answer, sources, and any errors. A human-readable
rendering is also emitted for quick debugging.

Usage::

    from src.observability import RequestLogger

    with RequestLogger() as logger:  # or logger = RequestLogger(); ...
        logger.start(question=query)
        ...
        logger.set_retrieval(context_chunks)
        ...
        logger.finish(answer=answer, usage=usage, error=None)
    # exits appending one JSON line per request to
    # data/logs/rag_requests.jsonl
"""

from __future__ import annotations

import json
import time
import uuid
from pathlib import Path
from typing import Any

from src.utils import get_logger

logger = get_logger(__name__)

DEFAULT_LOG_DIR = Path("data/logs")
DEFAULT_LOG_FILE = DEFAULT_LOG_DIR / "rag_requests.jsonl"


def new_request_id() -> str:
    """Short unique id for one request (e.g. abc123...)."""
    return uuid.uuid4().hex[:12]


class RequestLogger:
    """Collect per-request telemetry and append it to a JSONL log file."""

    def __init__(self, log_path: str | Path = DEFAULT_LOG_FILE) -> None:
        self.log_path = Path(log_path)
        self.record: dict[str, Any] = {}
        self._started_at: float = 0.0

    # -- lifecycle ------------------------------------------------------------
    def __enter__(self) -> "RequestLogger":
        return self

    def __exit__(self, *exc: object) -> None:
        self.flush()

    def start(self, question: str) -> "RequestLogger":
        """Begin a new request and stamp it with an id and timestamp."""
        self.record = {
            "request_id": new_request_id(),
            "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
            "question": question,
        }
        self._started_at = time.monotonic()
        return self

    def set_retrieval(self, chunks: list[dict[str, Any]]) -> "RequestLogger":
        """Record retrieved chunks with their retrieval scores."""
        self.record["retrieved_chunks"] = [
            {
                "chunk_id": chunk.get("chunk_id"),
                "source": chunk.get("source"),
                "score": chunk.get("score"),
            }
            for chunk in chunks
        ]
        self.record["retrieval_scores"] = [
            c.get("score") for c in chunks if c.get("score") is not None
        ]
        return self

    def set_prompt(self, prompt: str | list[dict[str, str]]) -> "RequestLogger":
        """Record the prompt (full text or the messages list) sent to the model."""
        self.record["prompt"] = prompt
        return self

    def set_model(self, model: str) -> "RequestLogger":
        self.record["model"] = model
        return self

    def set_memory(self, history_text: str) -> "RequestLogger":
        """Record the conversation history used for this request."""
        self.record["history"] = history_text
        return self

    def set_rewrite(self, rewritten_query: str) -> "RequestLogger":
        """Record the rewritten (standalone) retrieval query, if any."""
        self.record["rewritten_query"] = rewritten_query
        return self

    def add_guardrail(
        self, stage: str, passed: bool, message: str | None = None
    ) -> "RequestLogger":
        """Record an input/retrieval/generation guardrail outcome."""
        self.record.setdefault("guardrails", []).append(
            {"stage": stage, "passed": passed, "message": message}
        )
        return self

    def finish(
        self,
        answer: str,
        usage: dict[str, int | None] | None = None,
        sources: list[str] | None = None,
        error: str | None = None,
        latency: float | None = None,
    ) -> dict[str, Any]:
        """Complete the request: latency, answer, sources, and any error."""
        if latency is None and self._started_at:
            latency = round(time.monotonic() - self._started_at, 3)
        self.record["latency"] = latency
        self.record["answer"] = answer
        self.record["sources"] = sources or []
        self.record["usage"] = usage or {}
        self.record["error"] = error
        return self.record

    def record_error(self, error: Exception | str) -> "RequestLogger":
        """Mark the request as failed without losing prior fields."""
        self.record["error"] = str(error)
        return self

    # -- output ---------------------------------------------------------------
    def flush(self) -> None:
        """Append the current record to the JSONL log file (JSON log + readable)."""
        if not self.record:
            return
        if "latency" not in self.record and self._started_at:
            self.record["latency"] = round(time.monotonic() - self._started_at, 3)
        self.log_path.parent.mkdir(parents=True, exist_ok=True)
        with open(self.log_path, "a", encoding="utf-8") as fh:
            fh.write(json.dumps(self.record, ensure_ascii=False, default=str) + "\n")
        self._write_readable()
        logger.debug("Logged request %s", self.record.get("request_id"))
        self.record = {}

    # -- rendering ------------------------------------------------------------
    def render(self, record: dict[str, Any]) -> str:
        """Human-readable view in the style shown in the docs."""
        rid = record.get("request_id", "???")
        lines = [f"\nREQUEST {rid}", ""]
        lines.append(f'Question:\n{record.get("question", "")}')
        if record.get("rewritten_query"):
            lines.append(f'\nRewritten query:\n{record.get("rewritten_query")}')
        if record.get("history"):
            lines.append(f'\nConversation history:\n{record.get("history")}')
        retrieval = record.get("retrieved_chunks", [])
        if retrieval:
            lines.append("\nRetrieval:")
            lines.extend(
                f"  {chunk.get('chunk_id')}  score={chunk.get('score'):.3f}"
                for chunk in retrieval
                if chunk.get("score") is not None
            )
        lines.append(f'\nModel:\n{record.get("model", "")}')
        if record.get("latency") is not None:
            lines.append(f'\nLatency:\n{record.get("latency"):.2f}s')
        if record.get("usage"):
            usage = record["usage"]
            lines.append(
                "\nToken usage:\n  "
                + " | ".join(
                    f"{key}={value}" for key, value in usage.items() if value is not None
                )
            )
        lines.append(f'\nAnswer:\n{record.get("answer", "")}')
        sources = record.get("sources", [])
        if sources:
            lines.append("\nSources:")
            lines.extend(f"  {source}" for source in sources)
        if record.get("error"):
            lines.append(f'\nError:\n{record.get("error")}')
        guardrails = record.get("guardrails", [])
        if guardrails:
            lines.append("\nGuardrails:")
            lines.extend(
                f"  {g.get('stage')}: {'passed' if g.get('passed') else 'BLOCKED'}"
                + (f" - {g.get('message')}" if g.get("message") else "")
                for g in guardrails
            )
        return "\n".join(lines)

    def _write_readable(self) -> None:
        """Append the human-readable rendering next to the JSONL log."""
        readable_path = self.log_path.with_suffix(".log")
        with open(readable_path, "a", encoding="utf-8") as fh:
            fh.write(self.render(self.record) + "\n" + "=" * 60 + "\n")