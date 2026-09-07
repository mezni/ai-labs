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
mkdir -p src/utils src/llm
touch src/__init__.py app/__init__.py src/llm/__init__.py
cat > src/utils/__init__.py <<'EOF'
from src.utils.config_loader import load_all_configs, load_config, load_yaml_config
from src.utils.logger import get_logger

__all__ = ["get_logger", "load_config", "load_yaml_config", "load_all_configs"]
EOF
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


def load_config() -> dict:
    """Load the main LLM configuration file (config/llm_config.yaml)."""
    return load_yaml_config("config/llm_config.yaml")
EOF
cat > config/llm_config.yaml <<'EOF'
models:
  # Primary chat LLM used for copilot rationale / conversational responses.
  chat:
    provider: openrouter
    openrouter_url: https://openrouter.ai/api/v1/chat/completions
    model: minimax/MiniMax-M2
    max_tokens: 4096
    temperature: 0.3
    timeout_seconds: 120
EOF
cat > config/prompts.yaml <<'EOF'
prompts:
  query_rewrite:
    default_version: v1
    user_template: |-
      CONVERSATION HISTORY:
      {history}

      CURRENT QUESTION:
      {question}
    versions:
      v1:
        system: >-
          You prepare retrieval queries for a policy knowledge base. Given the
          conversation history and the current question, output a single
          standalone question that captures what the user is asking right now.
          Resolve pronouns ("it", "that", "this") and references using the
          history. Do not answer the question and do not repeat the history.
          Output only the rewritten question text with no quotes or prefix.

  retrieval_query:
    default_version: v2
    user_template: |-
      CONVERSATION:
      {history}

      CONTEXT:
      {context}

      QUESTION:
      {query}
    versions:
      v1:
        system: >-
          You answer questions using only the provided context. If the answer
          cannot be found in the context, say that you don't have enough
          information. The conversation history is only for resolving what the
          current question refers to; ground your answer in the context.
      v2:
        system: >-
          You are a grounded customer-support assistant. Answer using only the
          provided context; use the conversation history only to resolve what
          the current question refers to. Be concise and specific, quoting
          thresholds, prices, and windows where the context states them. If the
          context does not contain the answer, say that you don't have enough
          information.
EOF
cat > src/llm/llm_client.py <<'EOF'
"""OpenRouter chat client, configured from config/llm_config.yaml."""
import os
from typing import Any, Dict, List, Optional, Tuple

import httpx
from dotenv import load_dotenv

from src.utils import get_logger, load_config

load_dotenv()

logger = get_logger(__name__)


class LLMClient:
    """Generate responses by calling the LLM configured in llm_config.yaml.

    Chat, embedding, and reranker settings all come from config/llm_config.yaml.
    Requests are routed through the configured OpenRouter-compatible endpoints.
    """

    def __init__(
        self,
        model: Optional[str] = None,
        transport: Optional[httpx.BaseTransport] = None,
    ):
        config = load_config()
        chat_config = config.get("models", {}).get("chat", {})
        api_config = config.get("api", {})

        self.model = model or chat_config.get("model", "openai/gpt-4o-mini")
        self.temperature = chat_config.get("temperature", 0.3)
        self.max_tokens = chat_config.get("max_tokens", 4096)
        self.url = chat_config.get("openrouter_url", "https://openrouter.ai/api/v1/chat/completions")
        timeout = chat_config.get("timeout_seconds", api_config.get("timeout", 120))

        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")

        self._client = httpx.Client(
            transport=transport,
            timeout=timeout,
            headers={
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
            },
        )
        logger.debug(
            "LLMClient ready (model=%s, max_tokens=%d, temperature=%.2f)",
            self.model,
            self.max_tokens,
            self.temperature,
        )

    def generate(self, messages: List[Dict[str, str]], **kwargs) -> str:
        """Send the chat messages to the LLM and return the reply."""
        answer, _ = self.generate_with_usage(messages, **kwargs)
        return answer

    def generate_with_usage(
        self, messages: List[Dict[str, str]], **kwargs
    ) -> Tuple[str, Dict[str, Any]]:
        """Generate a reply and also return token usage (prompt/completion/total)."""
        logger.debug("Sending %d messages to %s", len(messages), self.model)
        response = self._client.post(
            self.url,
            json={
                "model": kwargs.get("model", self.model),
                "messages": messages,
                "max_tokens": kwargs.get("max_tokens", self.max_tokens),
                "temperature": kwargs.get("temperature", self.temperature),
            },
        )
        response.raise_for_status()
        data = response.json()
        answer = data["choices"][0]["message"]["content"]
        usage = data.get("usage", {})
        return answer, {
            "prompt_tokens": usage.get("prompt_tokens"),
            "completion_tokens": usage.get("completion_tokens"),
            "total_tokens": usage.get("total_tokens"),
        }

    def classify(self, query: str, system_prompt: str) -> str:
        """Classify user query using the chat model."""
        messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": query},
        ]
        return self.generate(messages)

    def close(self) -> None:
        self._client.close()
EOF
cat > src/llm/prompt_manager.py <<'EOF'
"""OpenRouter chat client, configured from config/llm_config.yaml."""
import os
from typing import Any, Dict, List, Optional, Tuple

import httpx
from dotenv import load_dotenv

from src.utils import get_logger, load_config

load_dotenv()

logger = get_logger(__name__)


class LLMClient:
    """Generate responses by calling the LLM configured in llm_config.yaml.

    Chat, embedding, and reranker settings all come from config/llm_config.yaml.
    Requests are routed through the configured OpenRouter-compatible endpoints.
    """

    def __init__(
        self,
        model: Optional[str] = None,
        transport: Optional[httpx.BaseTransport] = None,
    ):
        config = load_config()
        chat_config = config.get("models", {}).get("chat", {})
        api_config = config.get("api", {})

        self.model = model or chat_config.get("model", "openai/gpt-4o-mini")
        self.temperature = chat_config.get("temperature", 0.3)
        self.max_tokens = chat_config.get("max_tokens", 4096)
        self.url = chat_config.get("openrouter_url", "https://openrouter.ai/api/v1/chat/completions")
        timeout = chat_config.get("timeout_seconds", api_config.get("timeout", 120))

        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")

        self._client = httpx.Client(
            transport=transport,
            timeout=timeout,
            headers={
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
            },
        )
        logger.debug(
            "LLMClient ready (model=%s, max_tokens=%d, temperature=%.2f)",
            self.model,
            self.max_tokens,
            self.temperature,
        )

    def generate(self, messages: List[Dict[str, str]], **kwargs) -> str:
        """Send the chat messages to the LLM and return the reply."""
        answer, _ = self.generate_with_usage(messages, **kwargs)
        return answer

    def generate_with_usage(
        self, messages: List[Dict[str, str]], **kwargs
    ) -> Tuple[str, Dict[str, Any]]:
        """Generate a reply and also return token usage (prompt/completion/total)."""
        logger.debug("Sending %d messages to %s", len(messages), self.model)
        response = self._client.post(
            self.url,
            json={
                "model": kwargs.get("model", self.model),
                "messages": messages,
                "max_tokens": kwargs.get("max_tokens", self.max_tokens),
                "temperature": kwargs.get("temperature", self.temperature),
            },
        )
        response.raise_for_status()
        data = response.json()
        answer = data["choices"][0]["message"]["content"]
        usage = data.get("usage", {})
        return answer, {
            "prompt_tokens": usage.get("prompt_tokens"),
            "completion_tokens": usage.get("completion_tokens"),
            "total_tokens": usage.get("total_tokens"),
        }

    def classify(self, query: str, system_prompt: str) -> str:
        """Classify user query using the chat model."""
        messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": query},
        ]
        return self.generate(messages)

    def close(self) -> None:
        self._client.close()
EOF
