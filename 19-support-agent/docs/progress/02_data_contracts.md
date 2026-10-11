## Data contracts

### Goal

Make the ticket classifier accept validated input and produce structured, validated output using Pydantic.


### Problem Definition

Suppose your classifier returns this result:
{    "category": "order_status",    "order_id": 4821,  # Wrong type: integer, not string    "needs_order_lookup": "yes"  # Wrong type: string, not boolean}



A normal Python dictionary does not enforce your intended data structure. These mistakes can cause problems when your agent calls tools, sends data to an API, or processes an LLM response.
We want a contract that specifies:
- Which fields are required.
- Which types are allowed.
- Which values are valid.
- How data is serialized into JSON.
- How invalid input is rejected.
