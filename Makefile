export PYTHONPATH := src

.PHONY: install lint format test check db-up db-down migrate index

install:
	uv sync

lint:
	uv run ruff check .

format:
	uv run ruff format .

test:
	uv run pytest

check: lint test
	uv run ruff format --check .

db-up:
	docker compose up -d --wait db

db-down:
	docker compose down

migrate:
	uv run alembic upgrade head

index:
	uv run python -m indexing