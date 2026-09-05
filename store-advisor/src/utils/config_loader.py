import functools
from pathlib import Path
from typing import Any, Dict, Union

import yaml


@functools.lru_cache(maxsize=32)
def load_yaml(path: Union[str, Path]) -> Dict[str, Any]:
    """Load a YAML file and return its contents as a dict."""
    path = Path(path)
    if not path.is_file():
        raise FileNotFoundError(f"Config file not found: {path}")
    with open(path, encoding="utf-8") as f:
        data = yaml.safe_load(f)
    if not isinstance(data, dict):
        raise ValueError(f"Config root must be a mapping, got {type(data).__name__}: {path}")
    return data


def get_config(path: Union[str, Path], section: str) -> Dict[str, Any]:
    """Load a YAML config file and return one top-level section."""
    data = load_yaml(path)
    if section not in data:
        raise KeyError(f"Section '{section}' not found in {path}, available: {list(data)}")
    return data[section]