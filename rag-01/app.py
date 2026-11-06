from documents import documents
from retriever import retrieve
from generator import build_prompt


def run_rag(query: str):

    # 1. Retrieve relevant documents
    results = retrieve(query, documents)

    if not results:
        return {"answer": "I don't have sufficient information.", "sources": []}

    # 2. Build context and prompt
    prompt = build_prompt(query, results)

    # 3. Display the prompt for inspection
    print("\n--- GENERATED PROMPT ---")
    print(prompt)

    # 4. Return retrieved source information
    sources = [item["document"]["id"] for item in results]

    return {"prompt": prompt, "sources": sources}


if __name__ == "__main__":
    question = "What is the upload speed of Fiber 500?"

    response = run_rag(question)

    print("\n--- SOURCES ---")
    print(response["sources"])
