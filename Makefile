.PHONY: install lint format test check

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