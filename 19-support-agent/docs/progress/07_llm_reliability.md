## LLM Reliability, Timeouts, and Error Handling

### Goal
introduce explicit timeout and retry behavior, isolate the API client, and test failures without making real API calls.

### Problem Definition
Your current classifier works when everything goes right. But external API calls can fail because of:
- Network errors or connection timeouts.
- Rate limits or temporary server errors.
- Invalid credentials or unavailable models.
- Model refusals or missing parsed output.
A reliable AI application must handle these cases deliberately. It should not retry every error, silently misclassify a ticket, or wait indefinitely for an API response.

### Architecture
classify_ticket()
Validate input and coordinate inference

LLM client
Timeouts, bounded retries, API request

Validated TicketOutput
Or a clear exception when inference fails
