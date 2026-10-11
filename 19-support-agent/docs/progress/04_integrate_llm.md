## Connect Support Agent to an LLM

### Goal
Application will send a customer ticket to an LLM and receive a structured classification.


### Problem Definition
Current classify_ticket() function uses hard-coded keywords. It can misclassify tickets when customers describe the same issue in different ways.
For example:
- “Where is my order?”
- “The package I bought last week never showed up.”
- “Can you check when my delivery will arrive?”
A rule-based classifier needs many rules. An LLM can interpret the meaning of these messages.


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
