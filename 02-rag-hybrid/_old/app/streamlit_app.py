"""Streamlit Q&A frontend for the hybrid RAG pipeline."""
import sys
from pathlib import Path

import streamlit as st

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
CORPUS_DIR = ROOT / "data" / "policies"

from app.main import Generator, KnowledgeBase, Retriever, build_generation_prompt


def load_corpus_documents() -> list[dict[str, str]]:
    documents: list[dict[str, str]] = []
    for path in sorted(CORPUS_DIR.glob("*.md")):
        documents.append(
            {
                "document_id": path.stem,
                "source": path.name,
                "content": path.read_text(encoding="utf-8"),
            }
        )
    return documents


def ensure_components() -> None:
    if "retriever" in st.session_state:
        return
    with st.spinner("Building vector index..."):
        kb = KnowledgeBase()
        kb.build_index(load_corpus_documents())
        retriever = Retriever(kb.collection, kb.embedder, kb._chunks)
        try:
            generator = Generator()
        except RuntimeError:
            generator = None
        st.session_state.kb = kb
        st.session_state.retriever = retriever
        st.session_state.generator = generator


st.set_page_config(page_title="Aether Wireless KB Q&A", layout="wide")
st.title("Aether Wireless Knowledge Base Q&A")
st.caption("Hybrid retrieval (BM25 + embeddings) over policy documents")

ensure_components()

if st.session_state.generator is None:
    st.error(
        "OPENROUTER_API_KEY is not set. Add it to .env and restart the app "
        "to enable answer generation."
    )

with st.form("qa_form"):
    query = st.text_input("Ask a question about Aether Wireless policies")
    submitted = st.form_submit_button("Ask", type="primary")

if submitted and query.strip():
    with st.spinner("Thinking..."):
        chunks = st.session_state.retriever.retrieve(query.strip(), top_k=5)
        prompt = build_generation_prompt(query.strip(), chunks)
        answer_text = st.session_state.generator.generate(prompt)

    st.subheader("Answer")
    st.write(answer_text)

    with st.expander("Retrieved context"):
        for chunk in chunks:
            st.markdown(
                f"**{chunk['source']}** | {chunk['category']} | "
                f"score {chunk['score']:.3f}"
            )
            st.text(chunk["content"])
            st.divider()