from similarity import EmbeddingEngine


engine = EmbeddingEngine()

customer_questions = [
    "My internet connection is very slow.",
    "My broadband speed is poor.",
    "I cannot connect to Wi-Fi.",
    "I want to change my mobile subscription.",
    "My router keeps disconnecting.",
]

query = "My internet is not working properly."

results = engine.find_similar(
    query=query,
    documents=customer_questions,
    top_k=3,
)

print(f"Customer query: {query}\n")
print("Most similar questions:\n")

for result in results:
    print(f"Question: {result['document']}")
    print(f"Similarity: {result['similarity']:.4f}")
    print("-" * 50)
