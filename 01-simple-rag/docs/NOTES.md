# RAG Notes — Building a Simple Retrieval-Augmented Generation System

To build a simple Retrieval-Augmented Generation (RAG) system, you need to understand its core purpose: letting an LLM answer questions using your private or custom data rather than just its general training memory.

A standard ("Naive") RAG pipeline operates in two main phases: **Ingestion** (preparing the data) and **Inference** (retrieving data and generating an answer).

## Phase 1: Ingestion Pipeline (Data Setup)

Before you can search your documents, you must convert them into a machine-readable index.

1. **Load Documents** — Extract raw text from your files (PDFs, Markdown, database text, etc.).
2. **Chunking** — Split large texts into smaller, manageable blocks.
   - *Why:* LLMs have context limits, and searching smaller sections yields far better search precision.
   - *Standard Start:* Use recursive character splitting (~300–500 tokens per chunk with a ~50-token overlap to maintain context between splits).
3. **Embedding** — Pass each text chunk through an Embedding Model (e.g., `text-embedding-3-small`, HuggingFace `all-MiniLM-L6-v2`). This converts text into numerical vectors that represent semantic meaning.
4. **Vector Store** — Save these vectors alongside their raw text and source metadata (e.g., document title, page number) in a vector database.
   - *Beginner Options:* ChromaDB, FAISS, or pgvector (PostgreSQL).

## Phase 2: Inference Pipeline (Query & Generation)

When a user submits a question, the runtime pipeline executes:

1. **Query Embedding** — Convert the user's input text into a vector using the exact same embedding model from step 1.
2. **Retrieval (Vector Search)** — Perform a similarity search (like Cosine Similarity) in your vector database to find the Top-K (e.g., top 3–5) chunks closest in meaning to the query.
3. **Prompt Augmentation** — Combine the retrieved text chunks and the user query into a structured system prompt.
4. **Generation** — Send the augmented prompt to the LLM (e.g., GPT-4o, Claude, or local Llama) to generate a grounded response.

```
User Query ──► [Embed Model] ──► Query Vector
                                      │
                                      ▼
[Vector DB] ──(Top-K Chunks)──► [System Prompt] ──► [LLM] ──► Final Answer
```

## Key Prompt Template Structure

The prompt is the crucial glue that prevents the model from hallucinating outside your data.

```text
You are a helpful assistant. Answer the user's question ONLY using the context provided below.
If the context does not contain enough information to answer, state "I do not have enough information."

Context:
---
{retrieved_chunk_1}
---
{retrieved_chunk_2}

User Question: {user_query}
```

## Tools & Libraries to Use

- **Frameworks:** LangChain or LlamaIndex (These handle the heavy lifting of connecting models, splitters, and vector stores out of the box).
- **Vector DBs:** ChromaDB or FAISS (Local & in-memory, best for quick setup).
- **Embeddings & LLMs:** OpenAI API, Cohere, or local Ollama instances.

## Common Pitfalls to Avoid Early On

- **Poor Chunking:** If chunks are too small, they lose context; if too large, the vector search gets diluted with irrelevant information.
- **Mismatched Embedding Models:** Using Model A to create document vectors and Model B to process user queries will break the search completely.
- **Over-complicating early:** Do not start with complex workflows like agents or Knowledge Graphs. Build a basic vector search pipeline first, test it with real questions, and upgrade only when you spot specific retrieval failures.
