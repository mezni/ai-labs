"""Unit tests for config loading utilities."""

from __future__ import annotations

import pytest

from src.utils import load_config
from src.utils.config_loader import load_all_configs, load_yaml_config


def test_load_yaml_config_reads_file(tmp_path):
    config_file = tmp_path / "test.yaml"
    config_file.write_text("name: demo\nvalue: 42\n", encoding="utf-8")

    assert load_yaml_config(str(config_file)) == {"name": "demo", "value": 42}


def test_load_yaml_config_missing_file(tmp_path):
    missing = tmp_path / "nope.yaml"
    with pytest.raises(FileNotFoundError):
        load_yaml_config(str(missing))


def test_load_config_custom_path(tmp_path):
    config_file = tmp_path / "llm_config.yaml"
    config_file.write_text(
        "models:\n  chat:\n    model: minimax/MiniMax-M2\n",
        encoding="utf-8",
    )

    config = load_config(str(config_file))
    assert config["models"]["chat"]["model"] == "minimax/MiniMax-M2"


def test_load_config_real_config_file():
    """Sanity check against the repo's config/llm_config.yaml."""
    config = load_config("config/llm_config.yaml")
    chat = config["models"]["chat"]
    assert chat["provider"] == "openrouter"
    assert "model" in chat


def test_load_all_configs_skips_missing(tmp_path):
    (tmp_path / "llm_config.yaml").write_text(
        "models:\n  chat:\n    model: x\n",
        encoding="utf-8",
    )
    available = load_all_configs(str(tmp_path))
    assert "llm" in available
    assert "agent" not in available
    assert "prompts" not in available