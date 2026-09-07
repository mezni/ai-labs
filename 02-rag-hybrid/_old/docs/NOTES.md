# Hybrid Search

Hybrid search is a retrieval technique that combines traditional **lexical (keyword) search** with **vector (semantic) search** in a single pipeline. Instead of relying on only one search strategy, it runs both simultaneously and fuses their results into a single, higher-quality list.

## Why Use Hybrid Search?

Neither search method is perfect on its own. Combining them eliminates their individual blind spots:

| Search Type | Mechanism | Strengths | Weaknesses |
|-------------|-----------|-----------|------------|
| Lexical Search (e.g., BM25, TF-IDF) | Matches exact words and characters | Technical terms & SKUs, Proper nouns & names, Error codes | Misses synonyms, Cannot handle typos or rewording |
| Vector Search (Dense Embeddings) | Matches numerical context and meaning | Understands concepts, Handles synonyms/paraphrases, Cross-lingual matching | Struggles with exact codes, Can hallucinate semantic "closeness" for rare words |

## How It Works

```
                    ┌──► Lexical Search (BM25) ──► Exact Matches ──┐
User Query ──────────┤                                             ├──► Fusion Engine (RRF) ──► Final Top Results
                    └──► Vector Search ──────────► Concept Matches ┘
```

- **Parallel Execution:** When a query comes in, the database runs two separate searches in parallel: a BM25 algorithm searches for exact keyword matches in the text, while an embedding model calculates vector similarity to find semantic matches.
- **Result Fusion:** The system merges the two sets of results. Because raw BM25 scores (unbounded integers) and vector similarity scores (0.0 to 1.0) use different scales, databases typically use **Reciprocal Rank Fusion (RRF)**. RRF looks at document *rank positions* rather than raw scores to score and combine items fairly.
- **Alpha Weighting (Optional):** Many vector databases let you adjust an `alpha` parameter (e.g., `0.75` for 75% vector weight and 25% keyword weight) to tune the balance based on your application's needs.

## Real-World Example

If a user searches for **"Error 404 connection timeout"**:

1. **Vector Search** brings back general troubleshooting pages about network latency and server issues.
2. **Lexical Search** brings back pages that explicitly mention the exact string "404" and "timeout".
3. **Hybrid Search** prioritizes the specific "Error 404" troubleshooting document containing both the exact error code and the relevant network concepts.