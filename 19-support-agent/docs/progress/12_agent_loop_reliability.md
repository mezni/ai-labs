## Agent Loop Reliability and Failure Handling

### Goal
make the agent more reliable when the model or a tool returns unexpected results.

### Problem Definition
An agent can fail in several ways:
Failure	Example	Expected behavior
Invalid tool arguments	Order ID is an object instead of a string	Reject the arguments
Unknown tool	Model requests delete_order	Return a controlled error
Tool execution error	Order service raises an exception	Report a safe tool error
Repeated tool calls	Model keeps calling tools without answering	Stop at the round limit
Invalid model response	Missing choices or message	Fail clearly
Partial conversation turn	Tool loop fails halfway through	Avoid saving incomplete history
The goal is not to eliminate every possible failure. It is to make failures bounded, predictable, observable, and testable.


### Define the reliability contract
Before changing code, define what your agent promises:
- It never executes a tool outside its allowlist.
- It validates arguments before executing a tool.
- It limits the number of model/tool rounds.
- It does not expose internal exception details to the customer.
- It does not commit a failed turn to the saved conversation history.
- It records enough diagnostic information for developers to investigate failures.
These are agent reliability requirements, not just implementation details.

### Architecture
LLM response
Request a tool or return an answer

Validate tool name and arguments

Valid request
Execute the allowlisted tool and return a result.


Invalid or failed
Return a safe error result and log diagnostics.



Bounded agent loop
Continue, answer, or stop at the configured limit


One distinction matters: a tool returning an error does not necessarily mean the whole turn fails. The model may use that error to explain the problem. A model API exception or an exhausted loop, on the other hand, should fail the turn and leave the stored session history unchanged.