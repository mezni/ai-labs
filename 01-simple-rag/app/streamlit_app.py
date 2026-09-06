import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import requests
import streamlit as st
from dotenv import load_dotenv

from src.knowledge.embeddings import embed
from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever
from src.utils.config_loader import load_yaml_config
from src.utils.logger import get_logger

load_dotenv()

log = get_logger(__name__)

_config = load_yaml_config("config/llm_config.yaml")
_llm_cfg = _config["llm"]

LLM_URL = _llm_cfg["openrouter_url"]
LLM_MODEL = _llm_cfg["model"]
LLM_MAX_TOKENS = _llm_cfg["max_tokens"]
LLM_TEMPERATURE = _llm_cfg["temperature"]
LLM_TIMEOUT = _llm_cfg["timeout_seconds"]


@st.cache_resource
def load_kb() -> KnowledgeBase:
    return KnowledgeBase()


def llm_generate(prompt: str) -> str:
    response = requests.post(
        LLM_URL,
        headers={
            "Authorization": f"Bearer {os.environ['OPENROUTER_API_KEY']}",
            "Content-Type": "application/json",
        },
        json={
            "model": LLM_MODEL,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": LLM_MAX_TOKENS,
            "temperature": LLM_TEMPERATURE,
        },
        timeout=LLM_TIMEOUT,
    )
    response.raise_for_status()
    return response.json()["choices"][0]["message"]["content"]


st.set_page_config(page_title="Aether Wireless Policy RAG", layout="wide")
st.title("Aether Wireless Policy RAG")
st.markdown("Ask questions about Aether Wireless policies and get grounded answers.")

kb = load_kb()
retriever = Retriever(kb)

query = st.text_input("Ask a question:", placeholder="e.g. What happens if I exceed my data plan?")

if query:
    with st.spinner("Searching policies..."):
        prompt = retriever.build_prompt(query)
        results = retriever.retrieve(query)

    with st.spinner("Generating answer..."):
        answer = llm_generate(prompt)

    st.subheader("Answer")
    st.write(answer)

    with st.expander("Sources"):
        for i, r in enumerate(results, 1):
            st.markdown(f"**{i}. {r['filename']}** (score: {r['score']:.4f})")
            st.text(r["text"][:300] + ("..." if len(r["text"]) > 300 else ""))
            st.divider()