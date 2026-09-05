from pydantic import BaseModel, Field

import streamlit as st


class QuestionRequest(BaseModel):
    question: str = Field(..., min_length=1, max_length=2000)


class AnswerResponse(BaseModel):
    answer: str
    sources: list[str] = Field(default_factory=list)
    confidence: float = Field(default=0.0, ge=0.0, le=1.0)


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

        # TODO: call backend here
        # req = QuestionRequest(question=prompt)
        # resp: AnswerResponse = await backend.ask(req)
        # placeholder.markdown(resp.answer)

        answer = f"Echo: {prompt}"
        placeholder.markdown(answer)

    st.session_state.history.append({"role": "assistant", "content": answer})
