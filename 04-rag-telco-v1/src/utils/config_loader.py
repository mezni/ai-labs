"""Configuration loader for YAML files."""
import logging
from pathlib import Path

import yaml

logger = logging.getLogger(__name__)


def load_yaml_config(config_path: str) -> dict:
    """Load YAML configuration file."""
    path = Path(config_path)
    if not path.exists():
        raise FileNotFoundError(f"Config file not found: {config_path}")

    with open(path, "r", encoding="utf-8") as f:
        loaded = yaml.safe_load(f)
        return loaded if isinstance(loaded, dict) else {}


def load_all_configs(config_dir: str = "config") -> dict[str, dict]:
    """Load all available config files from the config/ directory.

    Missing optional configs (agent, prompts) are skipped with a warning so the
    loader works with only a subset of config files present.
    """
    config_dir = Path(config_dir)
    available: dict[str, dict] = {}
    for name, filename in [
        ("agent", "agent_config.yaml"),
        ("llm", "llm_config.yaml"),
        ("prompts", "prompts.yaml"),
    ]:
        path = config_dir / filename
        if path.exists():
            available[name] = load_yaml_config(path)
        else:
            logger.warning("Optional config %s not found, skipping", path)
    return available


def load_config(config_path: str = "config/llm_config.yaml") -> dict:
    """Load a YAML configuration file, defaulting to config/llm_config.yaml."""
    return load_yaml_config(config_path)
