import json
from pathlib import Path

from agents import name_generator_agent as nga
from agents.name_generator_agent import NameGeneratorAgent
from llm.llm_client import LLMResponse
from utils.tracer import Tracer


class FakeLLM:
    def __init__(self):
        self.config = {
            "model": "fake-model",
            "name_generation": {"max_idea_length": 1000},
            "pricing": {"input_per_million": 0.15, "output_per_million": 0.60},
        }

    def complete(self, messages):
        candidates = [
            {"name": f"Name{i}", "description": "d", "reason": "r"}
            for i in range(10)
        ]
        return LLMResponse(
            content=json.dumps({"candidates": candidates}),
            model="fake-model",
            input_tokens=120,
            output_tokens=90,
        )


def test_generate_records_expected_spans(tmp_path, monkeypatch, fake_llm=None):
    tracer = Tracer(traces_dir=tmp_path)
    monkeypatch.setattr(nga, "tracer", tracer)

    agent = NameGeneratorAgent(llm=FakeLLM())
    names = agent.generate("gym workout tracker")
    assert len(names) == 10
    assert names[0].name == "Name0"

    path = list(Path(tmp_path).glob("*.jsonl"))[0]
    records = [json.loads(line) for line in path.read_text().strip().splitlines()]
    operations = {r["operation"] for r in records}

    assert operations == {
        "generate_names",
        "validate_input",
        "build_prompt",
        "llm_call",
        "parse_response",
        "validate_output",
    }

    llm_call = next(r for r in records if r["operation"] == "llm_call")
    assert llm_call["model"] == "fake-model"
    assert llm_call["input_tokens"] == 120
    assert llm_call["output_tokens"] == 90
    assert llm_call["cost"] > 0
    assert all(r["status"] == "ok" for r in records)


def test_not_enabled_agent_tracing_writes_nothing(tmp_path, monkeypatch):
    tracer = Tracer(traces_dir=tmp_path, enabled=False)
    monkeypatch.setattr(nga, "tracer", tracer)

    agent = NameGeneratorAgent(llm=FakeLLM())
    agent.generate("gym workout tracker")

    assert not list(Path(tmp_path).iterdir())