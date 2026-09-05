import sys
from pathlib import Path
from typing import List

import streamlit as st
from pydantic import BaseModel, Field

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from src.llm.llm_client import LLMClient
from src.utils.logger import setup_logger

logger = setup_logger(__name__)


class QuestionRequest(BaseModel):
    question: str = Field(..., min_length=1, max_length=2000)


class AnswerResponse(BaseModel):
    answer: str
    sources: list[str] = Field(default_factory=list)
    confidence: float = Field(default=0.0, ge=0.0, le=1.0)


@st.cache_resource
def get_llm_client() -> LLMClient:
    return LLMClient()


st.set_page_config(page_title="Store Advisor", page_icon=":material/storefront:")
st.title("Store Advisor")

if "history" not in st.session_state:
    st.session_state.history: list[dict] = []

for msg in st.session_state.history:
    with st.chat_message(msg["role"]):
        st.markdown(msg["content"])

if prompt := st.chat_input("Ask a question about your store"):
    st.session_state.history.append({"role": "user", "content": prompt})
    with st.chat_message("user"):
        st.markdown(prompt)

    with st.chat_message("assistant"):
        placeholder = st.empty()
        placeholder.markdown("Thinking...")

        try:
            messages: List[dict] = [
                {"role": m["role"], "content": m["content"]}
                for m in st.session_state.history
            ]
            client = get_llm_client()
            answer = client.generate(messages)
            placeholder.markdown(answer)
        except Exception as e:
            logger.error("LLM call failed: %s", e)
            answer = f"Sorry, something went wrong: {e}"
            placeholder.markdown(answer)

    st.session_state.history.append({"role": "assistant", "content": answer})