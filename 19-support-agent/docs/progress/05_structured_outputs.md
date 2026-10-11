## Structured Outputs and Reliable LLM Responses

### Goal
Application will send a customer ticket to an LLM and receive a structured classification.


### Problem Definition
An LLM may return valid JSON that still violates your application's expectations. For example:
{
  "category": "shipping_problem",
  "order_id": 4821,
  "needs_order_lookup": "yes"
}


Your application expects:
- category to be one of order_status, refund, or general.
- order_id to be a string or null.
- needs_order_lookup to be a Boolean.
The model's response violates those expectations. Your application must validate model output rather than trust it.


# Architecture: Customer Ticket Classification Workflow

This diagram and flow breakdown illustrate how an incoming customer ticket is processed through an LLM and parsed into a structured response.

---

### System Architecture Diagram

+------------------+
| Customer Ticket  |  "My package never arrived."
+--------+---------+
|
v
+--------+---------+
| classify_ticket()|  App Logic & Orchestration
+--------+---------+
|
v
+--------+---------+
| Build Messages   |  Construct System Prompt & User Message
+--------+---------+
|
v
+--------+---------+
|  LLM Inference   |  API Call (OpenAI / OpenRouter / DeepSeek)
+--------+---------+
|
v
+--------+---------+
| Interpret Intent |  Structured JSON Extraction & Validation
+--------+---------+
|
v
+--------+---------+
|   TicketOutput   |  Typed Output Object / Schema
+------------------+


---

### Data Flow Breakdown

| Component | Role | Example Payload / Artifact |
| :--- | :--- | :--- |
| **Customer Ticket** | Raw user text input | `"My package never arrived."` |
| **`classify_ticket()`** | Entry function orchestrating the classification pipeline | Main application routine |
| **Build Messages** | Formats instructions, schemas, and user input into prompt format | `[{"role": "system", "content": "..."}, {"role": "user", ...}]` |
| **LLM Inference** | Remote or local model processing request via API | Provider response stream/JSON |
| **Interpret Intent** | Validates LLM output against target JSON schema | Schema parsing & error checking |
| **`TicketOutput`** | Final structured dataclass or Pydantic model | `{"category": "orde
