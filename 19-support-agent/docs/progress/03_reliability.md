## Engineering Quality and Testing

### Goal

Turn your ticket classifier into a maintainable, tested Python package that is ready for LLM integration Stage.


### Problem Definition
Your classifier works for ordinary tickets, but real customer messages can be ambiguous, unexpected, or malformed.
Examples:
- "Where is my order?" — no order ID provided.
- "I need a refund for order #4821" — two possible categories.
- "My order #4821 has not arrived" — straightforward order lookup.
- An empty string or an input of the wrong type — invalid input.
A production-oriented component should handle these cases consistently, expose its assumptions, and make failures easy to investigate.


### Improve the classifier
First, establish a deterministic policy for ambiguous tickets:
1. Refund requests take priority over order-status requests.
2. Order-status requests may include a missing order ID.
3. Unknown requests fall back to general.
4. Invalid input is rejected by the schema.
5. Logs record operational metadata, not the full customer message.
