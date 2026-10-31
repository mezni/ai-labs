from ccaf.llm import ClaudeClient
from ccaf.ticket_classifier import TicketClassifier

client = ClaudeClient()

classifier = TicketClassifier(client)

result = classifier.classify("My internet connection has stopped working.")

print(f"Category: {result}")
