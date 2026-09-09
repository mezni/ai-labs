"""Index Versions page — registry versions + configuration (PROJECT.md §19)."""

from __future__ import annotations

import pandas as pd
import streamlit as st

from app.artifacts import load_registry


def versions_frame(registry: dict) -> pd.DataFrame:
    versions = registry.get("versions", {})
    if not versions:
        return pd.DataFrame()
    current = registry.get("current_version")
    rows = [
        {
            "version": version,
            "current": "✓" if version == current else "",
            "collection": config.get("collection_name", ""),
            "chunks": config.get("chunk_count", 0),
            "documents": config.get("document_count", 0),
            "embedding_model": config.get("embedding_model", ""),
            "dimension": config.get("embedding_dimension", 0),
            "provider": config.get("embedding_provider", ""),
            "chunk_size": config.get("chunk_size", 0),
            "chunk_overlap": config.get("chunk_overlap", 0),
            "created": config.get("created_at", ""),
        }
        for version, config in versions.items()
    ]
    return pd.DataFrame(rows)


def render(registry: dict) -> None:
    st.title("Index Versions")
    st.caption("Every registry version + its reproducible configuration.")

    frame = versions_frame(registry)
    if frame.empty:
        st.info("No index versions recorded — run `rag-dataops index` first.")
        return

    current = registry.get("current_version")
    st.markdown(f"**Current version:** {current or '—'}")

    st.dataframe(frame, use_container_width=True)

    st.subheader("Version configuration")
    version = st.selectbox("Inspect version", list(registry.get("versions", {})))
    config = registry["versions"][version]
    st.json(config)


def main() -> None:
    render(load_registry())


if __name__ == "__main__":
    main()