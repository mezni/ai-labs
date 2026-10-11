## Workflow Routing and Conditional Branches

### Goal
separate routing decisions from step execution so the workflow becomes easier to extend and test.

### Problem Definition
Your current workflow mixes classification, branching, tool execution, and response generation in one function. As you add more ticket categories, that function will become harder to maintain.
For example, a support agent may need to handle:
- Order status — retrieve order information.
- Refund requests — retrieve refund policy.
- General questions — search support documentation.
- Missing information — ask the customer a clarifying question.
- Unsupported requests — return a controlled response.
We'll introduce a routing function that determines the next workflow action from the current state.



### Architecture
Validate and classify

Route using explicit rules

Order lookup
Order ID available


Knowledge search
Policy or general topic


Clarification
Required details missing


Finish
No action required




The important distinction is that routing decides what should happen; execution performs the operation.