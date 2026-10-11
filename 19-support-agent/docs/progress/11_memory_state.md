## Conversation Memory and State

### Goal
preserve the conversation history and reuse it for follow-up questions, while keeping the existing order-status and knowledge-search tools.

### Problem Definition
Currently, handle_ticket(ticket) creates a new messages list every time it runs. The agent can answer an individual ticket, but it may lose context between messages.
Example:

Customer — Message 1
“Where is order #4821?”



Agent
“Order #4821 has shipped. The estimated delivery date is October 12, 2026.”



Customer — Message 2
“What about its delivery date?”



The second message does not contain the order number. Without conversation history, the agent may not know what “its” refers to.



### Understand today's engineering concepts
Conversation history: the messages sent to and returned by the model.



Session state: data that persists across multiple turns in one conversation.



Message roles: system, user, assistant, and tool.



Tool-call history: preserve the assistant's tool-call message and the corresponding tool results.



Session isolation: one customer's conversation must not leak into another customer's session.