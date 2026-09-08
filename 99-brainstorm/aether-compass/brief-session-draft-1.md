# Brief — Aether Compass (Draft 1)

**Status:** in-progress (brainstorming session, `2026-09-08`) · **Owner:** Aether Wireless

## Idea in one line
An AI assistant that answers telecom policy and procedure questions for
customer-service employees, grounded in the company's *approved* documents as
the single source of truth.

## Problem / Why
Customer-service employees need fast, accurate answers to telecom policy
questions, but the relevant information is distributed across many documents,
**versions, sections, and procedures**. Delays and wrong answers cost customers
and the company.

## Core requirements so far
- Fast answers, on the clock (agents have seconds, not minutes).
- Accurate and trustworthy answers — grounded in approved, *current* company
  documents, not general knowledge.
- Must handle scattered + versioned sources: many documents, multiple versions,
  cross-referenced sections and procedures.

## Query corpus (raw, from session)
| Type | Query |
|---|---|
| point | What is the policy for cancelling a mobile contract? |
| point | How long does a customer have to request a refund? |
| point | What documents are required for SIM replacement? |
| point | What is the international roaming policy? |
| point | When can a customer request number portability? |
| point | What is the current policy for service suspension? |
| conditional | Does this policy apply to prepaid customers? |
| procedure | What is the procedure for a billing dispute? |
| procedure | What is the escalation procedure for a disputed charge? |

**Observation:** queries fall into two kinds — *point-answers* and
*conditional-answers* — and the conditional ones are where the
versioned/scattered-document pain bites hardest.

## Open questions
- How many queries (and which kinds) are in the target corpus?
- How are documents versioned, and how does "approved/current" get determined
  and enforced?
- What is the *why* behind the assistant? (reducing support load / training /
  audit / single source of truth) — not fully pinned yet.

## Source
BMad brainstorming session memlog:
`brainstorming/brainstorm-telecom-policy-assistant-2026-09-08/.memlog.md`