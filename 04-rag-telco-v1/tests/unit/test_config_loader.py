"""Unit tests for src.utils.config_loader."""
import pytest
import yaml

from src.utils.config_loader import load_all_configs, load_yaml_config


def test_load_yaml_config(tmp_path):
    config_file = tmp_path / "test.yaml"
    config_file.write_text("chat:\n  model: test-model\n", encoding="utf-8")

    result = load_yaml_config(str(config_file))

    assert result == {"chat": {"model": "test-model"}}


def test_load_yaml_config_missing_file(tmp_path):
    missing = tmp_path / "missing.yaml"

    with pytest.raises(FileNotFoundError):
        load_yaml_config(str(missing))


def test_load_yaml_config_empty_returns_dict(tmp_path):
    empty_file = tmp_path / "empty.yaml"
    empty_file.write_text("", encoding="utf-8")

    result = load_yaml_config(str(empty_file))

    assert result == {}


def test_load_all_configs_skips_missing(tmp_path, caplog):
    (tmp_path / "llm_config.yaml").write_text(
        yaml.safe_dump({"provider": "openrouter"}),
        encoding="utf-8",
    )

    result = load_all_configs(str(tmp_path))

    assert "llm" in result
    assert "prompts" not in result and "agent" not in result
    assert any("skipping" in r.message for r in caplog.records)