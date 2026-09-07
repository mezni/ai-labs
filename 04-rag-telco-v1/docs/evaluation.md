# Evaluation

## Purpose

Measure whether the RAG system retrieves the right policy evidence and produces answers that are grounded, correct, and correctly cited.

## Evaluation Dataset

The project must include a dedicated evaluation dataset. Each item records the question, the expected source, and the expected answer.

```json
{
  "question": "What is the refund period?",
  "expected_document": "refund-policy.pdf",
  "expected_section": "4.1",
  "expected_answer": "..."
}
```

Target: **50–100 policy questions** covering the policy categories:

- Mobile plans
- Billing
- Refunds
- Contract cancellation
- SIM replacement
- Number portability
- Roaming
- Customer verification
- Complaints
- Payment procedures
- Service activation
- Service suspension
- Escalation procedures

## Metrics

### Retrieval

- Did the system retrieve the correct policy document?
- Did the system retrieve the correct section?

### Relevance

- Are the retrieved chunks relevant to the question?
- Precision and recall at top-K (Hit Rate, MRR).

### Groundedness

- Is the generated answer supported by the retrieved context?
- Answer must not contain facts outside the provided chunks.

### Correctness

- Does the answer correctly answer the question?

### Citation Accuracy

- Does the cited source (document, section, page) actually support the answer?

## Guardrail Checks

- **No-answer behavior**: unsupported questions yield the no-evidence message.
- **Version filtering**: obsolete policy versions are not cited when a current version exists.
- **Conflict detection**: conflicting active sources trigger the review message instead of a silent answer.

## Observability Data for Evaluation

Each request should capture reusable telemetry for analysis:

```text
Request ID
Timestamp
User question

Retrieved documents
Retrieved chunks
Similarity scores

Embedding latency
Retrieval latency
LLM latency

Model
Input tokens
Output tokens

Final answer
Sources
Errors
```

## Success Criteria (Evaluation Relevant)

1. Relevant policy chunks can be retrieved.
2. The LLM generates answers based on retrieved evidence.
3. Answers include reliable citations.
4. Unsupported questions produce an appropriate no-answer response.
5. Obsolete policies are excluded.
6. Retrieval and generation can be evaluated.