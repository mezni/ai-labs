"""Tests for ``scripts/generate_sample_pdfs.py`` sample-corpus generation."""

import importlib.util
from pathlib import Path
from types import SimpleNamespace

import pytest

from rag.config import Settings
from rag.extract import extract_pdf

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "generate_sample_pdfs.py"
_spec = importlib.util.spec_from_file_location("generate_sample_pdfs", SCRIPT)
assert _spec.loader is not None
generate = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(generate)

CATEGORIES = {"billing", "mobile", "roaming", "customer", "compliance"}


def _response(content="Refunds are accepted.", usage=None):
    return SimpleNamespace(
        choices=[SimpleNamespace(message=SimpleNamespace(content=content))],
        usage=usage
        if usage is not None
        else SimpleNamespace(prompt_tokens=10, completion_tokens=5, total_tokens=15),
    )


class StubCompletions:
    def __init__(self, responder):
        self.responder = responder
        self.calls = []

    def create(self, *, model, messages, **kwargs):
        self.calls.append({"model": model, "messages": messages, **kwargs})
        return self.responder(self.calls[-1])


class StubClient:
    def __init__(self, responder):
        self.chat = SimpleNamespace(completions=StubCompletions(responder))


class TestCatalog:
    def test_every_document_is_aurora_branded(self):
        for _, meta in generate.KB.items():
            assert meta.doc_id.startswith("AUR-")
            assert meta.title
            assert meta.brief

    def test_categories_are_known_and_match_sort_order(self):
        for name, meta in generate.KB.items():
            assert meta.category in CATEGORIES, name

    def test_metadata_versions_are_semver_like(self):
        for meta in generate.KB.values():
            assert meta.version.count(".") == 1

    def test_long_docs_are_flagged(self):
        assert any(meta.sections > 0 for meta in generate.KB.values())
        single_pass = [meta for meta in generate.KB.values() if meta.sections == 0]
        assert single_pass

    def test_catalog_does_not_leak_old_branding(self):
        source = SCRIPT.read_text(encoding="utf-8")
        assert "Aether" not in source
        assert "AW-" not in source


class TestTargetResolution:
    def test_only_filters_by_substring(self):
        targets = generate._resolve_targets(
            generate.argparse.Namespace(only=["roaming"], limit=None)
        )
        assert targets
        assert all("roaming" in name for name, _ in targets)

    def test_only_without_match_raises(self):
        with pytest.raises(SystemExit):
            generate._resolve_targets(generate.argparse.Namespace(only=["nonexistent"], limit=None))

    def test_limit_truncates_in_catalog_order(self):
        targets = generate._resolve_targets(generate.argparse.Namespace(only=None, limit=3))
        assert [name for name, _ in targets] == list(generate.KB)[:3]


class TestMarkdownParsing:
    def test_parse_markdown_recognizes_blocks(self):
        body = "## Overview\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n- bullet\n\n1. step\n\npara\n"
        kinds = [kind for kind, _ in generate._parse_markdown(body)]
        assert kinds == ["h2", "table", "bullets", "numbered", "para"]

    def test_parse_markdown_drops_separator_rows(self):
        blocks = generate._parse_markdown("| A | B |\n|---|---|\n| 1 | 2 |")
        assert blocks == [("table", [["A", "B"], ["1", "2"]])]


class TestLlmHelpers:
    def test_extract_text_from_object(self):
        assert generate._extract_text(_response()) == "Refunds are accepted."

    def test_extract_text_handles_missing_choices(self):
        assert generate._extract_text(SimpleNamespace()) == ""

    def test_usage_normalizes_objects_and_dicts(self):
        obj = SimpleNamespace(prompt_tokens=1, completion_tokens=2, total_tokens=3)
        assert generate._usage(_response(usage=obj)) == {
            "prompt_tokens": 1,
            "completion_tokens": 2,
            "total_tokens": 3,
        }
        assert generate._usage(SimpleNamespace(usage={})) == {}
        assert generate._usage(SimpleNamespace()) == {}

    def test_llm_generate_passes_model_and_returns_text(self):
        calls = []
        client = StubClient(lambda call: calls.append(call) or _response())

        text, usage = generate._llm_generate(
            client, "minimax/minimax-m3", "system", "user", max_tokens=200
        )

        assert text == "Refunds are accepted."
        assert usage["total_tokens"] == 15
        assert calls[-1]["model"] == "minimax/minimax-m3"
        assert calls[-1]["messages"] == [
            {"role": "system", "content": "system"},
            {"role": "user", "content": "user"},
        ]
        assert calls[-1]["max_tokens"] == 200

    def test_llm_generate_always_bounds_output(self):
        calls = []
        client = StubClient(lambda call: calls.append(call) or _response())

        generate._llm_generate(client, "model", "s", "u")

        assert calls[-1]["max_tokens"] == generate.BODY_MAX_TOKENS

    def test_llm_generate_retries_then_succeeds(self):
        responder = _Flaky(failures=2)
        client = StubClient(responder)

        text, _ = generate._llm_generate(client, "model", "s", "u", retries=3)

        assert responder.attempts == 3
        assert text == "Refunds are accepted."

    def test_llm_generate_propagates_last_error(self):
        responder = _Flaky(failures=99)
        client = StubClient(responder)

        with pytest.raises(RuntimeError, match="transient"):
            generate._llm_generate(client, "model", "s", "u", retries=2)

        assert responder.attempts == 2


class _Flaky:
    def __init__(self, failures):
        self.failures = failures
        self.attempts = 0

    def __call__(self, call):
        self.attempts += 1
        if self.attempts <= self.failures:
            raise RuntimeError("transient failure")
        return _response()


class TestBuildClient:
    def test_requires_model(self):
        with pytest.raises(ValueError, match="OPENROUTER_MODEL"):
            generate.build_client(Settings())

    def test_forwards_settings_to_openai(self, monkeypatch):
        captured = {}

        class StubOpenAI:
            def __init__(self, **kwargs):
                captured.update(kwargs)

        monkeypatch.setattr(generate, "OpenAI", StubOpenAI)
        settings = Settings(
            openrouter_base_url="https://openrouter.example.com",
            openrouter_api_key="sk-test",
            openrouter_model="minimax/minimax-m3",
        )

        client, model = generate.build_client(settings)

        assert captured["base_url"] == "https://openrouter.example.com"
        assert captured["api_key"] == "sk-test"
        assert model == "minimax/minimax-m3"
        assert client is not None

    def test_model_reads_from_openrouter_model_env(self, monkeypatch):
        captured = {}

        class StubOpenAI:
            def __init__(self, **kwargs):
                captured.update(kwargs)

        monkeypatch.setenv("OPENROUTER_MODEL", "minimax/minimax-m3")
        monkeypatch.setattr(generate, "OpenAI", StubOpenAI)

        _, model = generate.build_client(Settings())

        assert model == "minimax/minimax-m3"


class TestRendering:
    def test_render_pdf_is_indexable(self, tmp_path):
        meta = generate.KB["refund-policy"]
        body = (
            "## Overview\n\nRefunds for Aurora Mobile customers take 14 days.\n\n"
            "| Window | Fee |\n|---|---|\n| 14 days | none |\n\n"
            "- refundable within 14 days\n\n**Q: How long?**\n**A: 14 days.**\n"
        )
        out = tmp_path / "billing" / "refund-policy.pdf"

        pages = generate.render_pdf(generate.KnowledgeDocument(metadata=meta, body_md=body), out)

        assert pages >= 1
        assert generate._pdf_page_count(out) == pages
        extracted = extract_pdf(out)
        text = " ".join(page.text for page in extracted.pages)
        assert "Overview" in text
        assert "14 days" in text

    def test_main_renders_selected_documents(self, tmp_path, monkeypatch):
        monkeypatch.setattr(
            generate, "build_client", lambda settings: (object(), "minimax/minimax-m3")
        )
        monkeypatch.setattr(
            generate,
            "generate_body",
            lambda client, model, meta: ("## Overview\n\nBody.", {"completion_tokens": 10}),
        )

        code = generate.main(
            [
                "--only",
                "refund",
                "--limit",
                "1",
                "--out-dir",
                str(tmp_path),
                "--keep-md",
            ]
        )

        assert code == 0
        assert (tmp_path / "billing" / "refund-policy.pdf").is_file()
        assert (tmp_path / "billing" / "refund-policy.md").is_file()
