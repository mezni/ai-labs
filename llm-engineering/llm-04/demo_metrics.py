from metrics import InferenceMetrics


metrics = InferenceMetrics(
    request_start=0.0,
    first_token_time=1.2,
    generation_end=6.2,
    generated_tokens=100,
)

print("LLM INFERENCE PERFORMANCE")
print("-------------------------")

for metric, value in metrics.report().items():
    print(f"{metric}: {value}")
