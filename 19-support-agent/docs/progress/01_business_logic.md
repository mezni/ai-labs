## Business logic

### Problem Definition

When a customer submits a support ticket, the system must process the unformatted text, extract key entity information, and return a structured JSON payload for downstream services.

---

### Example Input
> **Ticket Text:**  
> *"My order #4821 has not arrived. Can you help?"*

---

### Expected Output
```json
{
  "category": "order_status",
  "order_id": "4821",
  "needs_order_lookup": true
}
