"""Streamlit chat UI for the Aether Wireless policy RAG copilot.

Run:
    uv run streamlit run app/streamlit_app.py
"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import streamlit as st

from src.guardrails import (
    NO_RELEVANT_MESSAGE,
    UNGROUNDED_MESSAGE,
    EmptyQuestionError,
    check_grounding,
    check_retrieval,
    validate_query,
)
from src.knowledge import Reranker
from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever
from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.observability import RequestLogger
from src.utils import get_logger

logger = get_logger(__name__)

CORPUS_DIR = Path("data/policies")
INDEX_PATH = Path("data/faiss/index.bin")
CHUNKS_PATH = Path("data/faiss/chunks.json")


@st.cache_resource(show_spinner="Loading knowledge base...")
def load_kb(force_rebuild: bool = False) -> KnowledgeBase:
    """Load the cached FAISS index or rebuild it from the corpus."""
    kb = KnowledgeBase(
        corpus_dir=CORPUS_DIR,
        index_path=INDEX_PATH,
        chunks_path=CHUNKS_PATH,
    )
    cached = INDEX_PATH.exists() and CHUNKS_PATH.exists()
    if force_rebuild or not cached:
        documents = kb.load_documents()
        kb.build_index(documents)
        kb.save_index()
    else:
        kb.load_index()
    return kb


@st.cache_resource(show_spinner="Preparing retriever...")
def load_retriever(
    _kb: KnowledgeBase,
    use_rerank: bool,
    sparse_top_k: int,
    dense_top_k: int,
) -> Retriever:
    reranker = Reranker() if use_rerank else None
    return Retriever(
        _kb,
        sparse_top_k=sparse_top_k,
        dense_top_k=dense_top_k,
        reranker=reranker,
    )


def build_messages(query: str, context: list[dict], prompt_version: str | None) -> list[dict[str, str]]:
    """Assemble system + user messages for the LLM from the prompt template."""
    prompt = PromptManager().get_prompt("retrieval_query", version=prompt_version)
    user = PromptManager().format_prompt(
        prompt["user_template"],
        context="\n\n".join(f"[{c['source']}]\n{c['content']}" for c in context),
        query=query,
    )
    messages: list[dict[str, str]] = []
    if prompt.get("system"):
        messages.append({"role": "system", "content": prompt["system"]})
    messages.append({"role": "user", "content": user})
    return messages


st.set_page_config(page_title="Aether Wireless Policy RAG", layout="wide")
st.title("Aether Wireless Policy RAG")
st.markdown("Ask questions about Aether Wireless policies and get grounded answers.")

# -- sidebar ---------------------------------------------------------
with st.sidebar:
    st.subheader("Retrieval settings")
    use_rerank = st.toggle("Use reranker", value=False, help="Re-score fused candidates.")
    sparse_top_k = st.slider("BM25 candidates", 5, 30, 15)
    dense_top_k = st.slider("FAISS candidates", 5, 30, 15)
    top_k = st.slider("Context chunks", 1, 10, 5)
    prompt_version = st.selectbox("Prompt version", ["v1", "v2"])
    if st.button("Rebuild index from corpus", use_container_width=True):
        st.session_state["force_rebuild"] = True
        load_kb.clear()
        load_retriever.clear()
        st.rerun()

try:
    kb = load_kb(st.session_state.pop("force_rebuild", False))
except (RuntimeError, FileNotFoundError) as exc:
    st.error(f"Knowledge base unavailable: {exc}")
    st.stop()

if not kb.is_built():
    st.warning(
        f"No documents found in {CORPUS_DIR.resolve()}. "
        "Run `uv run python scripts/generate_docs.py` first."
    )
    st.stop()

retriever = load_retriever(kb, use_rerank, sparse_top_k, dense_top_k)

# -- chat ------------------------------------------------------------
query = st.chat_input("Ask a question about Aether Wireless policies...")

if query:
    st.chat_message("user").write(query)

    try:
        query = validate_query(query)
    except EmptyQuestionError as exc:
        st.error(str(exc))
    else:
        with st.chat_message("assistant"):
            with st.spinner("Searching policies..."):
                context = retriever.retrieve(query, top_k=top_k)
            retrieval_refusal = check_retrieval(context)
            if retrieval_refusal is not None:
                with RequestLogger() as logger:
                    logger.start(question=query)
                    logger.set_retrieval(context)
                    logger.add_guardrail("input", True)
                    logger.add_guardrail("retrieval", False, retrieval_refusal)
                    logger.finish(answer=retrieval_refusal, sources=[])
                st.warning(retrieval_refusal)
            else:
                with st.spinner("Generating answer..."):
                    try:
                        messages = build_messages(query, context, prompt_version)
                        with RequestLogger() as logger:
                            logger.start(question=query)
                            logger.set_retrieval(context)
                            logger.set_prompt(messages)
                            logger.add_guardrail("input", True)
                            logger.add_guardrail("retrieval", True)
                            client = LLMClient()
                            logger.set_model(client.model)
                            try:
                                answer, usage = client.generate_with_usage(messages)
                            except Exception as exc:
                                logger.record_error(exc)
                                raise
                            finally:
                                client.close()

                            context_text = "\n\n".join(
                                f"[{c['source']}]\n{c['content']}" for c in context
                            )
                            grounding_refusal = check_grounding(answer, context_text)
                            if grounding_refusal is not None:
                                logger.add_guardrail("generation", False, grounding_refusal)
                                logger.finish(
                                    answer=grounding_refusal,
                                    usage=usage,
                                    sources=sorted({c["source"] for c in context}),
                                )
                                st.warning(grounding_refusal)
                            else:
                                logger.add_guardrail("generation", True)
                                logger.finish(
                                    answer=answer,
                                    usage=usage,
                                    sources=sorted({c["source"] for c in context}),
                                )
                            final_answer = (
                                grounding_refusal
                                if grounding_refusal is not None
                                else answer
                            )
                    except RuntimeError as exc:
                        st.error(f"Generation failed: {exc}")
                        st.stop()
                st.write(final_answer)

                with st.expander("Sources"):
                    for i, chunk in enumerate(context, 1):
                        st.markdown(f"**{i}. {chunk['source']}** (score: {chunk['score']:.4f})")
                        st.text(chunk["content"][:300] + ("..." if len(chunk["content"]) > 300 else ""))
                        st.divider()