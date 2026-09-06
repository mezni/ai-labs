#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <project-name>" >&2
    exit 1
fi

mkdir -p "$1"
cd "$1"
uv init
uv venv
touch .env .env.example
cat > .env.example <<'EOF'
# Application environment configuration
# Copy to .env and fill in your values. Never commit the real .env file.

# Logging level: DEBUG, INFO, WARNING, ERROR
LOG_LEVEL=INFO

# OpenRouter API key for LLM access
OPENROUTER_API_KEY=
EOF
cp .env.example .env
printf '.env\n.venv/\n__pycache__/\n' > .gitignore
mkdir -p app scripts src docs
touch app/main.py
mkdir -p config
mkdir -p src/utils
touch src/__init__.py src/utils/__init__.py app/__init__.py
cat > src/utils/logger.py <<'EOF'
import logging
import os

from dotenv import load_dotenv

load_dotenv()

LOG_LEVEL = os.getenv("LOG_LEVEL", "INFO").upper()

LOG_FORMAT = "%(asctime)s | %(levelname)-8s | %(name)s | %(message)s"
LOG_DATE_FORMAT = "%Y-%m-%d %H:%M:%S"

_initialized = False


def get_logger(name: str) -> logging.Logger:
    global _initialized
    if not _initialized:
        logging.basicConfig(
            level=LOG_LEVEL,
            format=LOG_FORMAT,
            datefmt=LOG_DATE_FORMAT,
        )
        _initialized = True
    return logging.getLogger(name)
EOF
cat > src/utils/config_loader.py <<'EOF'
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
        return yaml.safe_load(f)


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
EOF
cat > config/llm_config.yaml <<'EOF'
models:
  - name: default
    provider: openai
    model: gpt-4o
    temperature: 0.7
EOF
