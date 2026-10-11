## Tool Calling

### Goal
introduce tool calling: the LLM can request an application function, your Python code executes it, and the result is returned to the model.

### Problem Definition
A customer asks:
My order #4821 hasn't arrived. Can you help?

Your current classifier can identify order_status and extract 4821. But it cannot determine whether the order has shipped.
We'll create a tool called get_order_status(order_id) that retrieves order information from a small mock database.

### Architecture
Customer ticket
“Where is order #4821?”

LLM
Requests get_order_status

Python tool executor
Validates arguments and executes the function

Mock order database
Returns status and estimated delivery


Important: the LLM can request a tool call, but your Python application decides whether and how to execute it. The model does not execute Python code itself.
