from analyzer import (
    TokenizerAnalyzer,
    ContextBudget,
)


analyzer = TokenizerAnalyzer()

message = """
My internet connection is not working.
I restarted my router but the problem persists.
Can you help me?
"""

result = analyzer.analyze(message)

print("TOKEN ANALYSIS")
print("-------------------")
print("Characters:", result["character_count"])
print("Tokens:", result["token_count"])
print("Token IDs:", result["token_ids"])


budget = ContextBudget(
    context_limit=32000,
    reserved_output_tokens=4000,
)

remaining = budget.calculate_remaining(result["token_count"])

print("\nCONTEXT BUDGET")
print("-------------------")
print("Context limit:", budget.context_limit)
print("Reserved output:", budget.reserved_output_tokens)
print("Input tokens:", result["token_count"])
print("Remaining:", remaining)
print("Can fit:", budget.can_fit(result["token_count"]))
