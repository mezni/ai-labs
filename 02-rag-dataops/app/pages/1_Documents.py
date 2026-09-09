"""Documents page — the document catalog (PROJECT.md §19)."""

from __future__ import annotations

import pandas as pd
import streamlit as st

from app.artifacts import load_catalog


def catalog_frame(catalog: dict) -> pd.DataFrame:
    if not catalog:
        return pd.DataFrame()
    rows = [
        {
            "document": doc_id,
            "version": record.get("version", ""),
            "hash": (record.get("hash", "") or "")[:12],
            "last_modified": record.get("last_modified", ""),
            "source": record.get("source", ""),
            "format": record.get("format", ""),
            "chunks": len(record.get("chunk_ids", [])),
        }
        for doc_id, record in catalog.items()
    ]
    return pd.DataFrame(rows)


def render(catalog: dict) -> None:
    st.title("Documents")
    st.caption("Document catalog: versions, hashes, metadata, and lineage.")

    frame = catalog_frame(catalog)
    if frame.empty:
        st.info("Catalog is empty — run `rag-dataops index` first.")
        return

    st.subheader("Filters")
    col_a, col_b = st.columns(2)
    sources = ["All"] + sorted(set(frame["source"]))
    formats = ["All"] + sorted(set(frame["format"]))
    source_filter = col_a.selectbox("Source", sources)
    format_filter = col_b.selectbox("Format", formats)

    filtered = frame
    if source_filter != "All":
        filtered = filtered[filtered["source"] == source_filter]
    if format_filter != "All":
        filtered = filtered[filtered["format"] == format_filter]

    st.dataframe(filtered, use_container_width=True)
    st.caption(f"{len(filtered)} documents")


def main() -> None:
    render(load_catalog())


if __name__ == "__main__":
    main()