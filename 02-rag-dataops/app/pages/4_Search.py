"""Search page — top-k retrieval with lineage metadata (PROJECT.md §19).

Calls the same service as the CLI: ``src/retrieval/retriever.py``.
"""

from __future__ import annotations

import streamlit as st

from src.config import get_settings
from src.retrieval.retriever import RetrievalError, Retriever


def render() -> None:
    st.title("Search")
    st.caption("Top-k chunks with lineage metadata from the current snapshot.")

    settings = get_settings()
    if not settings.openrouter_api_key:
        st.warning("OPENROUTER_API_KEY is not set — search needs it to embed the query.")
        return

    query = st.text_input("Question", placeholder="e.g. what is the refund policy?")
    top_k = st.slider("Top-k", 1, 20, settings.top_k)

    if not query or not st.button("Search"):
        return

    retriever = Retriever(settings, _embedder(settings))
    try:
        results = retriever.retrieve(query, top_k=top_k)
    except RetrievalError as exc:
        st.error(str(exc))
        return

    if not results:
        st.info("No results.")
        return

    for chunk in results:
        with st.container(border=True):
            st.markdown(f"**{chunk.chunk_id}** — score `{chunk.score}`")
            meta = chunk.lineage
            st.caption(
                f"document {meta.get('document_id')} · version {meta.get('version')} · "
                f"source {meta.get('source')} · format {meta.get('format')} · "
                f"index snapshot {meta.get('index_version')}"
            )
            st.markdown(chunk.text)


def _embedder(settings):
    from src.embeddings.embedder import Embedder

    return Embedder(
        api_key=settings.openrouter_api_key,
        base_url=settings.openrouter_base_url,
        model=settings.openrouter_embedding_model,
    )


def main() -> None:
    render()


if __name__ == "__main__":
    main()