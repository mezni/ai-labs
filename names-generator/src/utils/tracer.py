import json
import os
import time
import uuid
from contextlib import contextmanager
from datetime import datetime, timezone
from pathlib import Path

DEFAULT_TRACES_DIR = Path(__file__).resolve().parents[2] / "traces"


class Span:
    def __init__(self, operation: str, attributes: dict):
        self.operation = operation
        self.attributes = attributes

    def add(self, **kwargs):
        self.attributes.update(kwargs)


class Tracer:
    def __init__(self, traces_dir: str | Path = DEFAULT_TRACES_DIR, enabled: bool = True):
        self.traces_dir = Path(traces_dir)
        self.enabled = enabled
        self._trace_id = None
        self._depth = 0

    def _prepare(self):
        self.traces_dir.mkdir(parents=True, exist_ok=True)
        if self._trace_id is None:
            self._trace_id = str(uuid.uuid4())

    @contextmanager
    def span(self, operation: str, **attributes):
        if not self.enabled:
            yield Span(operation, attributes)
            return

        self._prepare()
        self._depth += 1
        span = Span(operation, attributes)
        started = time.perf_counter()
        status = "ok"

        try:
            yield span
        except Exception as error:
            status = "error"
            span.add(error=str(error))
            raise
        finally:
            duration_ms = (time.perf_counter() - started) * 1000
            self._write(
                {
                    "timestamp": datetime.now(timezone.utc).isoformat(),
                    "trace_id": self._trace_id,
                    "operation": operation,
                    "status": status,
                    "duration_ms": round(duration_ms, 2),
                    **span.attributes,
                }
            )
            self._depth -= 1
            if self._depth == 0:
                self._trace_id = None

    def _write(self, record: dict):
        date = datetime.now(timezone.utc).strftime("%Y-%m-%d")
        path = self.traces_dir / f"{date}.jsonl"
        with open(path, "a") as f:
            f.write(json.dumps(record) + "\n")


tracer = Tracer(enabled=os.getenv("CATALYST_TRACING", "1") != "0")