"""Lineage page — drill from a chunk to its document/version/source/snapshot.

Lineage comes from the document catalog (PROJECT.md §8): each chunk id embeds
``{document_id}::{version}::chunk::{index}`` and the catalog maps the document
to its source connector and snapshot membership.
"""

from __future__ import annotations

import re
from typing import Any

import pandas as pd
import streamlit as st

from app.artifacts import load_catalog, load_registry

_CHUNK_PATTERN = re.compile(r"^(?P<document_id>.+)::(?P<version>v\d+)::chunk::(?P<index>\d+)$")


def lookup_lineage(chunk_id: str, catalog: dict, registry: dict) -> dict[str, Any]:
    """Trace chunk → document → version → source → snapshot (PROJECT.md §8)."""
    match = _CHUNK_PATTERN.match(chunk_id)
    if not match:
        return {"chunk_id": chunk_id, "error": "unrecognized chunk id format"}
    document_id = match.group("document_id")
    record = catalog.get(document_id, {})
    versions = registry.get("versions", {})
    lineage: dict[str, Any] = {
        "chunk_id": chunk_id,
        "document_id": document_id,
        "version": match.group("version"),
        "chunk_index": match.group("index"),
        "source": record.get("source", "unknown"),
        "format": record.get("format", "unknown"),
        "last_modified": record.get("last_modified", ""),
        "chunk_ids": record.get("chunk_ids", []),
        "current_index_version": registry.get("current_version"),
        "snapshots": [],
    }
    for version, config in versions.items():
        if document_id in {c.split("::v", 1)[0] for c in record.get("chunk_ids", [])}:
            lineage["snapshots"].append(
                {
                    "index_version": version,
                    "collection": config.get("collection_name", ""),
                    "embedding_model": config.get("embedding_model", ""),
                    "embedding_dimension": config.get("embedding_dimension", 0),
                }
            )
            break
    return lineage


def render(catalog: dict, registry: dict) -> None:
    st.title("Lineage")
    st.caption("Chunk → document → version → source connector → index snapshot.")

    all_chunks = [
        chunk_id
        for record in catalog.values()
        for chunk_id in record.get("chunk_ids", [])
    ]
    if not all_chunks:
        st.info("No chunks recorded yet — run `rag-dataops index` first.")
        return

    selected = st.selectbox("Chunk", all_chunks)
    lineage = lookup_lineage(selected, catalog, registry)
    st.json(lineage)

    st.subheader("Trail")
    trail = pd.DataFrame(
        [
            {"hop": "chunk", "value": lineage["chunk_id"]},
            {"hop": "document", "value": lineage["document_id"]},
            {"hop": "document version", "value": lineage["version"]},
            {"hop": "source connector", "value": lineage["source"]},
            {
                "hop": "index snapshot",
                "value": lineage["snapshots"][0]["index_version"]
                if lineage["snapshots"]
                else lineage["current_index_version"],
            },
            {
                "hop": "embedding model",
                "value": lineage["snapshots"][0]["embedding_model"]
                if lineage["snapshots"]
                else "-",
            },
        ]
    )
    st.dataframe(trail, use_container_width=True)


def main() -> None:
    render(load_catalog(), load_registry())


if __name__ == "__main__":
    main()