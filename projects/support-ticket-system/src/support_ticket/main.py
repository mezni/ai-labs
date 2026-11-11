from support_ticket.repositories.ticket_repository import TicketRepository
from support_ticket.services.ticket_service import TicketService


def main():

    repository = TicketRepository()

    service = TicketService(repository)

    ticket = service.create_ticket("My order #4821 has not arrived.")

    print(ticket.model_dump())


if __name__ == "__main__":
    main()
