import os
from collections.abc import Iterator
from pathlib import Path

import pytest
from alembic import command
from alembic.config import Config
from sqlalchemy import Engine, create_engine, text
from sqlalchemy.exc import OperationalError

ROOT = Path(__file__).resolve().parent.parent
TEST_URL = os.environ.get(
    "TEST_DATABASE_URL", "postgresql+psycopg://postgres:postgres@localhost:5432/rag_test"
)


@pytest.fixture(scope="session")
def engine() -> Iterator[Engine]:
    engine = create_engine(TEST_URL)
    try:
        with engine.connect():
            pass
    except OperationalError:
        engine.dispose()
        if os.environ.get("CI"):
            raise  # in CI a missing database is a failure, not a skip
        pytest.skip("Postgres is not reachable: run `make db-up`")

    config = Config(str(ROOT / "alembic.ini"))
    config.set_main_option("sqlalchemy.url", TEST_URL)
    command.upgrade(config, "head")
    yield engine
    engine.dispose()


@pytest.fixture
def clean_db(engine: Engine) -> Engine:
    with engine.begin() as connection:
        connection.execute(text("TRUNCATE app.chunks CASCADE"))
    return engine
