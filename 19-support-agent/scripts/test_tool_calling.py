from dotenv import load_dotenv

from support_agent.tool_calling import handle_ticket

load_dotenv()

if __name__ == "__main__":
    ticket = input("Customer ticket: ").strip()

    if not ticket:
        raise SystemExit("Please enter a ticket.")

    try:
        print("\nAgent response:")
        print(handle_ticket(ticket))
    except Exception as exc:
        print(f"Request failed: {type(exc).__name__}: {exc}")
        raise SystemExit(1)
