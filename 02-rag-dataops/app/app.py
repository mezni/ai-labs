"""RAG DataOps dashboard — Overview page.

Usage: streamlit run app/app.py
"""

from __future__ import annotations

import pandas as pd
import streamlit as st

from app.artifacts import load_catalog, load_registry, source_counts


def registry_frame(registry: dict) -> pd.DataFrame:
    versions = registry.get("versions", {})
    if not versions:
        return pd.DataFrame()
    rows = [
        {
            "version": version,
            "collection": config.get("collection_name", ""),
            "chunks": config.get("chunk_count", 0),
            "documents": config.get("document_count", 0),
            "model": config.get("embedding_model", ""),
            "dim": config.get("embedding_dimension", 0),
        }
        for version, config in versions.items()
    ]
    return pd.DataFrame(rows)


def render(registry: dict, catalog: dict) -> None:
    st.set_page_config(page_title="RAG DataOps", page_icon="🚀", layout="wide")
    st.title("RAG DataOps — Overview")
    st.caption("Index versions, snapshots, document catalog, lineage and rollback.")

    current = registry.get("current_version")
    versions = registry.get("versions", {})
    current_config = versions.get(current, {})

    col_a, col_b, col_c = st.columns(3)
    col_a.metric("Current index version", current or "—")
    col_b.metric("Chunks in current snapshot", current_config.get("chunk_count", 0))
    col_c.metric(
        "Documents in current snapshot", current_config.get("document_count", 0)
    )

    counts = source_counts(registry)
    if counts:
        st.subheader("Chunks per source")
        st.bar_chart(pd.Series(counts))

    st.subheader("Index versions")
    frame = registry_frame(registry)
    if frame.empty:
        st.info("No index versions recorded yet — run `rag-dataops index`.")
    else:
        st.dataframe(frame, use_container_width=True)

    if not catalog:
        st.info("Document catalog is empty — run `rag-dataops index`.")


def main() -> None:
    render(load_registry(), load_catalog())


if __name__ == "__main__":
    main()