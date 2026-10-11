## Add a Knowledge-Base Search Tool

### Goal
give your support-agent a second tool: search_knowledge_base(query). This allows the agent to answer questions using approved help-center documents rather than relying entirely on the LLM's general knowledge.

### Problem Definition
A customer asks:
What is your return policy?

Your current agent can look up orders, but it has no trusted source for return policies, shipping rules, or account guidance.
We'll add a small local knowledge base containing approved support articles. The agent will be able to choose between:
- get_order_status(order_id) — retrieve order information.
- search_knowledge_base(query) — retrieve relevant help-center content.
For this lab, the knowledge base is a small in-memory dataset. We'll explore embeddings and vector search in a later phase.

### Architecture
Customer ticket

LLM agent loop
Chooses the appropriate tool

Order lookup
Existing tool

Knowledge search
New tool


Grounded customer response
Based on retrieved data or documents


### Understand today's engineering concepts
- Knowledge base: a curated collection of documents the agent can consult.
- Retrieval: selecting relevant documents for a user's question.
- Grounded generation: generating an answer based on retrieved evidence.
- Lexical search: matching terms in a query against document text.
- Semantic search: retrieving conceptually relevant documents even when their wording differs.
- Tool registry: the set of functions the agent is permitted to call.