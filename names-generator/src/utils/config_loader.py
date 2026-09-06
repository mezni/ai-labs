from pathlib import Path
import os

import yaml

DEFAULT_CONFIG_PATH = Path(__file__).resolve().parents[2] / "config" / "llm_config.yaml"
DEFAULT_PROMPTS_PATH = Path(__file__).resolve().parents[2] / "config" / "prompts.yaml"
DEFAULT_AGENT_CONFIG_PATH = Path(__file__).resolve().parents[2] / "config" / "agent_config.yaml"

ENV_OVERRIDES = {
    "model": "MODEL",
}


def load_yaml(path: str | Path) -> dict:
    with open(path) as f:
        return yaml.safe_load(f)


def load_llm_config(path: str | Path = DEFAULT_CONFIG_PATH) -> dict:
    config = load_yaml(path)
    if "model" not in config:
        raise ValueError(f"Missing 'model' in LLM config: {path}")

    for key, env_name in ENV_OVERRIDES.items():
        value = os.getenv(env_name)
        if value:
            config[key] = value

    return config


def load_prompts(path: str | Path = DEFAULT_PROMPTS_PATH) -> dict:
    prompts = load_yaml(path)
    if "name_generation" not in prompts:
        raise ValueError(f"Missing 'name_generation' in prompts config: {path}")
    return prompts


def load_agent_config(path: str | Path = DEFAULT_AGENT_CONFIG_PATH) -> dict:
    config = load_yaml(path)
    name_generator = config.get("name_generator")
    if name_generator is None:
        raise ValueError(f"Missing 'name_generator' in agent config: {path}")

    overrides = {
        "max_items": os.getenv("NUMBER_OF_NAMES"),
        "max_attempts": os.getenv("MAX_ATTEMPTS"),
    }
    for key, value in overrides.items():
        if value:
            name_generator[key] = int(value)

    return config