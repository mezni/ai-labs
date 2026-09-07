"""Configuration loader for YAML files."""
from pathlib import Path

import yaml

from src.utils.logger import get_logger

logger = get_logger(__name__)


def load_yaml_config(config_path: str) -> dict:
    """Load YAML configuration file."""
    path = Path(config_path)
    if not path.exists():
        raise FileNotFoundError(f"Config file not found: {config_path}")

    with open(path, "r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def load_all_configs(config_dir: str = "config") -> dict[str, dict]:
    """Load available YAML config files from the config/ directory.

    `llm` is the active backend config; other configs are loaded when their
    YAML file exists. Optional-missing files are logged at debug level only.
    """
    config_dir = Path(config_dir)
    available: dict[str, dict] = {}
    for name, filename in [
        ("llm", "llm_config.yaml"),
        ("prompts", "prompts.yaml"),
        ("agent", "agent_config.yaml"),
    ]:
        path = config_dir / filename
        if path.exists():
            available[name] = load_yaml_config(path)
        else:
            logger.debug("Optional config %s not found, skipping", path)
    return available