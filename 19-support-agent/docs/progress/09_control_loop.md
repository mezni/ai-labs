## Controlled Agent Loop

### Goal
you'll turn that one-shot interaction into a controlled agent loop that can process tool calls, return results to the LLM, and continue until it has a final answer.

### Problem Definition
Consider this ticket:
Check order #4821 and tell me its delivery status.

The LLM might request get_order_status. Your application executes the tool and sends its result back to the model.
But a more complex ticket might need several tools. Your agent needs a loop that can handle repeated requests without running forever or executing arbitrary functions.

### Architecture
1. Send conversation to LLM

2. Did it request tools?

No
Return answer
Finish

Yes
Validate and execute
Append tool results


3. Send updated conversation to LLM
Repeat until finished or the limit is reached


The loop is controlled by your Python code. The LLM chooses from the tools you expose, but your application enforces execution rules.



### Understand the engineering trade-offs
Before moving on, make sure you can explain these concepts:
- Agent loop: repeated model requests and tool executions until a stopping condition is reached.
- Tool registry: the application's allowlist of callable functions.
- Tool-call correlation: matching each tool result to its request using tool_call_id.
- Bounded execution: limiting rounds to avoid unbounded calls and costs.
- Error isolation: returning controlled tool errors instead of exposing internal exception details to the model.