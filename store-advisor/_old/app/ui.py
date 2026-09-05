import os

import httpx
import streamlit as st
from dotenv import load_dotenv

load_dotenv()

API_BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8000")

if "api_client" not in st.session_state:
    st.session_state.api_client = httpx.Client(base_url=API_BASE_URL, timeout=120)


def ask(message: str) -> str:
    r = st.session_state.api_client.post("/chat", json={"message": message})
    r.raise_for_status()
    return r.json()["reply"]


def main() -> None:
    st.set_page_config(page_title="Store Advisor", layout="centered")
    st.title("Store Advisor")
    st.caption(f"Backend: {API_BASE_URL}")

    if "messages" not in st.session_state:
        st.session_state.messages = []

    for msg in st.session_state.messages:
        with st.chat_message(msg["role"]):
            st.write(msg["content"])

    if prompt := st.chat_input("Ask about products, pricing, or store policies"):
        st.session_state.messages.append({"role": "user", "content": prompt})
        with st.chat_message("user"):
            st.write(prompt)

        with st.chat_message("assistant"):
            with st.spinner("Thinking..."):
                try:
                    reply = ask(prompt)
                except httpx.HTTPStatusError as e:
                    st.error(f"API error: {e.response.status_code} {e.response.text}")
                    st.stop()
                except httpx.RequestError:
                    st.error(f"Cannot reach the API at {API_BASE_URL}. Start it with: uv run uvicorn app.main:app")
                    st.stop()
                st.write(reply)
        st.session_state.messages.append({"role": "assistant", "content": reply})


if __name__ == "__main__":
    main()