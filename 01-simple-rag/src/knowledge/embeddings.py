import os
import time

import requests

from src.utils.config_loader import load_yaml_config
from src.utils.logger import get_logger

log = get_logger(__name__)

_config = load_yaml_config("config/llm_config.yaml")
_emb_cfg = _config["embeddings"]
_chunk_cfg = _config["chunking"]

OPENROUTER_URL = _emb_cfg["openrouter_url"]
EMBEDDING_MODEL = _emb_cfg["model"]
EMBEDDING_TIMEOUT = _emb_cfg["timeout_seconds"]

MAX_RETRIES = 3
RETRY_BACKOFF = 2


def chunk_text(
    text: str,
    chunk_size: int = _chunk_cfg["chunk_size"],
    overlap: int = _chunk_cfg["overlap"],
) -> list[str]:
    if overlap >= chunk_size:
        raise ValueError(f"overlap ({overlap}) must be less than chunk_size ({chunk_size})")

    chunks = []
    start = 0
    while start < len(text):
        end = start + chunk_size
        chunks.append(text[start:end])
        start = end - overlap
    return chunks


def embed(texts: list[str]) -> list[list[float]]:
    last_error: Exception | None = None
    for attempt in range(1, MAX_RETRIES + 1):
        try:
            response = requests.post(
                OPENROUTER_URL,
                headers={
                    "Authorization": f"Bearer {os.environ['OPENROUTER_API_KEY']}",
                    "Content-Type": "application/json",
                },
                json={
                    "model": EMBEDDING_MODEL,
                    "input": texts,
                },
                timeout=EMBEDDING_TIMEOUT,
            )
            response.raise_for_status()
            data = response.json()
            return [item["embedding"] for item in data["data"]]
        except requests.exceptions.HTTPError as e:
            last_error = e
            if response.status_code == 429:
                wait = RETRY_BACKOFF * attempt
                log.warning(
                    "Rate limited (429), retrying in %ds (attempt %d/%d)",
                    wait, attempt, MAX_RETRIES,
                )
                time.sleep(wait)
            else:
                break
        except requests.exceptions.ConnectionError as e:
            last_error = e
            wait = RETRY_BACKOFF * attempt
            log.warning(
                "Connection error, retrying in %ds (attempt %d/%d)",
                wait, attempt, MAX_RETRIES,
            )
            time.sleep(wait)

    raise RuntimeError(f"Embedding failed after {MAX_RETRIES} attempts: {last_error}")