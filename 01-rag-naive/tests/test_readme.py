"""Tests that the README and CI workflow stay consistent with the project."""

import re
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = Path(__file__).resolve().parents[2]
README = (PROJECT_ROOT / "README.md").read_text(encoding="utf-8")


class TestReadMe:
    def test_module_list_matches_package(self):
        line = next(line for line in README.splitlines() if "src/rag/" in line)
        modules = re.split(r"[,\s]+", line.split("#", 1)[1].strip())
        for module in modules:
            assert (PROJECT_ROOT / "src" / "rag" / f"{module}.py").is_file()

    def test_documents_cli_subcommands(self):
        for subcommand in ("index", "reindex", "search", "ask"):
            assert subcommand in README

    def test_references_existing_scripts(self):
        assert "scripts/generate_sample_pdfs.py" in README
        assert (PROJECT_ROOT / "scripts" / "generate_sample_pdfs.py").is_file()

    def test_pointers_match_project_layout(self):
        assert "docs/PROJECT.md" in README
        assert "docs/TODO.md" in README
        assert ".env.example" in README
        assert (PROJECT_ROOT / "docs" / "PROJECT.md").is_file()
        assert (PROJECT_ROOT / "docs" / "TODO.md").is_file()
        assert (PROJECT_ROOT / ".env.example").is_file()


class TestCiWorkflow:
    def test_workflow_exists_and_gates_project(self):
        workflow = REPO_ROOT / ".github" / "workflows" / "01-rag-naive-ci.yml"
        assert workflow.is_file()

        content = workflow.read_text(encoding="utf-8")
        assert "working-directory: 01-rag-naive" in content
        assert "uv sync" in content
        assert "uv run ruff check src tests" in content
        assert "uv run pytest -q" in content
