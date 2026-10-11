## Prompt Engineering and Evaluation

### Goal
create a labeled evaluation dataset, run the classifier against it, calculate accuracy, and improve the prompt based on measured errors.


### Problem Definition
Your LLM can return valid structured output and still classify a ticket incorrectly.
For example:
- “I want my money back for order #4821.” → refund
- “Can you tell me when order #4821 will arrive?” → order_status
- “How can I update my account email?” → general
A schema validates the shape of the answer, not whether the answer is correct.

### Metrics
- Category accuracy: fraction of cases where the category matches the expected category.
- Exact-match accuracy: fraction where all three output fields match.
- Failed calls: cases where the classifier raised an exception instead of returning a result.
For example, a ticket can have the correct category but the wrong order_id. That counts as category-correct, but not an exact match.
