import os

import streamlit as st
from dotenv import load_dotenv

from agents.name_generator_agent import NameGeneratorAgent
from llm.llm_client import LLMConfigError

load_dotenv()

st.set_page_config(page_title="Blueprint Generator", layout="centered")

st.title("Blueprint Generator")
st.markdown(
    "Transform a project idea into a technical blueprint.\n\n"
    "**Step 1 — Generate project names.**"
)

idea = st.text_area("Enter your IT project idea", height=100)

if st.button("Generate names"):
    if not idea.strip():
        st.warning("Please enter a project idea.")
        st.stop()

    if not os.getenv("OPENROUTER_API_KEY"):
        st.error("OPENROUTER_API_KEY is not set. Add it to your .env file.")
        st.stop()

    with st.spinner("Generating names..."):
        try:
            agent = NameGeneratorAgent()
            candidates = agent.generate(idea)
        except (LLMConfigError, ValueError, RuntimeError) as error:
            st.error(f"Unable to generate names: {error}")
            st.stop()

    st.success(f"Generated {len(candidates)} project names")

    for index, candidate in enumerate(candidates, start=1):
        st.divider()
        st.subheader(f"{index}. {candidate.name}")
        st.write(candidate.description)
        st.caption(f"Why: {candidate.reason}")