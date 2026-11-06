def build_prompt(query: str, retrieved_documents: list) -> str:
    context = "\n\n".join(item["document"]["content"] for item in retrieved_documents)

    prompt = f"""
You are a telecom support assistant.

Answer the question using only the provided context.

If the answer is not available, say:
"I don't have sufficient information."

Context:
{context}

Question:
{query}

Answer:
"""

    return prompt
