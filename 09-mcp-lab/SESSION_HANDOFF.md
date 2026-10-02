# Session Handoff

Last updated: 2026-09-24

## Project status

`mcp-lab` is a fresh MCP (Model Context Protocol) server project, Python 3.12+, managed with `uv`.

- Version: `0.1.0`
- Package: `src/mcp_lab/` (layout set with `uv`/src layout in `pyproject.toml`)
- Dependency: `mcp[cli]>=2.2.0`

## Current state

- Committed on `main` (`3a94ae9`): `pyproject.toml`, `uv.lock`, `.python-version`, `README.md`.
- Committed after `3a94ae9`, uncommitted working tree contains the MCP server work.
- `src/mcp_lab/server.py`: `MCPServer` ("mcp-lab") with a single `add(a, b)` tool, stdio transport.
- `src/mcp_lab/__init__.py`: empty.
- `tests/test_server.py`: tests `add(2, 3) == 5`; passing.
- `pyproject.toml`: added hook `pytest` dev dep, `[build-system]` (hatchling), and wheel package config for `src/mcp_lab`.
- `src/` and `tests/` are currently **untracked** in git (not yet committed).
- `README.md` exists but is empty.

## Git note

The repo history contains many older unrelated commits ahead of the cleaned-up history that ends at `3a94ae9 Create project`. Treat `main` HEAD(`3a94ae9`) as the true starting point.

## Next steps

1. Fill in `src/mcp_lab/__init__.py` (package metadata / exports).
2. Add more tools/resources; consider an entrypoint in `[project.scripts]` (`mcp-lab`).
3. Introduce an MCP client to verify protocol end-to-end.
4. Commit the working tree and keep `CHANGELOG.md` updated.

## Commands

- Sync env: `uv sync`
- Run tests: `uv run pytest`
- Run server (stdio): `echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}' | uv run python src/mcp_lab/server.py`