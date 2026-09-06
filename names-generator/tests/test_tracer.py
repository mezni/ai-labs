import json
from pathlib import Path

import pytest

from utils.tracer import Tracer


def _lines(records: str):
    return [json.loads(line) for line in records.strip().splitlines()]


def test_single_span_writes_jsonl(tmp_path):
    tracer = Tracer(traces_dir=tmp_path)
    with tracer.span("build_prompt", idea="gym app"):
        pass

    date_path = list(Path(tmp_path).glob("*.jsonl"))[0]
    records = _lines(date_path.read_text())
    assert len(records) == 1
    record = records[0]
    assert record["operation"] == "build_prompt"
    assert record["status"] == "ok"
    assert record["idea"] == "gym app"
    assert "duration_ms" in record
    assert "trace_id" in record


def test_nested_spans_share_trace_id(tmp_path):
    tracer = Tracer(traces_dir=tmp_path)
    with tracer.span("generate_names"):
        with tracer.span("validate_input"):
            pass
        with tracer.span("llm_call", model="test-model"):
            pass

    path = list(Path(tmp_path).glob("*.jsonl"))[0]
    records = _lines(path.read_text())
    assert len(records) == 3
    assert len({r["trace_id"] for r in records}) == 1


def test_span_records_error(tmp_path):
    tracer = Tracer(traces_dir=tmp_path)
    with pytest.raises(RuntimeError):
        with tracer.span("llm_call"):
            raise RuntimeError("boom")

    path = list(Path(tmp_path).glob("*.jsonl"))[0]
    record = _lines(path.read_text())[0]
    assert record["status"] == "error"
    assert record["error"] == "boom"


def test_disabled_tracer_writes_nothing(tmp_path):
    tracer = Tracer(traces_dir=tmp_path, enabled=False)
    with tracer.span("llm_call"):
        pass

    assert not list(Path(tmp_path).iterdir())