## Workflow State and Explicit Agent Steps

### Goal
A conversation history tells you what has been said. Workflow state tells you what the system has done, what it knows, and what should happen next.

### Problem Definition
Consider this support request:
“My order #4821 hasn't arrived. Can you help?”

Your agent needs to perform several operations:
1. Validate the ticket.
2. Classify the request.
3. Look up the order.
4. Interpret the result.
5. Generate a customer-facing response.
Currently, much of that process is implicit in the LLM's decisions. That makes it harder to inspect, test, and control the overall workflow.



### Architecture
Ticket input
Raw customer message

1. Validate
Check the input schema

2. Classify
Determine category and order ID

3. Execute workflow
Look up order, retrieve policy, or answer generally

4. Produce result
Structured state and final response


The key design decision: the workflow controls the sequence of operations; the LLM supports tasks that benefit from language understanding. We will not ask the LLM to decide every operational step.