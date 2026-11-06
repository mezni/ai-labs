import re


def tokenize(text: str) -> set[str]:
    return set(re.findall(r"\b\w+\b", text.lower()))


def retrieve(query: str, documents: list, top_k: int = 2):
    query_tokens = tokenize(query)

    results = []

    for document in documents:
        content_tokens = tokenize(document["content"])

        score = len(query_tokens & content_tokens)

        results.append({"document": document, "score": score})

    results.sort(key=lambda item: item["score"], reverse=True)

    return [item for item in results[:top_k] if item["score"] > 0]
