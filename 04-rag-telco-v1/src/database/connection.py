"""PostgreSQL database connection."""

import os

import psycopg
from dotenv import load_dotenv

load_dotenv()


class DatabaseConnection:
    """Manage PostgreSQL connections."""

    def __init__(self, database_url: str | None = None) -> None:
        self.database_url = database_url or os.getenv("DATABASE_URL")

        if not self.database_url:
            raise RuntimeError("DATABASE_URL is not set")

    def connect(self) -> psycopg.Connection:
        """Create and return a PostgreSQL connection."""
        return psycopg.connect(self.database_url)