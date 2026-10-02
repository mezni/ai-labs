# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version History

| Version | Feature Domain | Key Objectives |
| --- | --- | --- |
| `0.1.1` (Unreleased) | MCP server, packaging, testing | Add `MCPServer` with `add` tool; hatchling build backend/package install; `pytest` test suite |
| `0.1.0` | Project scaffold | Bootstrap `mcp-lab` with `uv`, src layout, `mcp[cli]` dependency |

---

## [Unreleased]

### Added

- MCP server with an `add` tool in `src/mcp_lab/server.py`, using `MCPServer` (mcp 2.x API).
- `tests/test_server.py` with `pytest` as a dev dependency.
- Build backend (`hatchling`) and package config so `src/mcp_lab` is installed as an editable package.

## [0.1.0] - 2026-09-24

### Added

- Project scaffold: `pyproject.toml`, `uv.lock`, `.python-version`, `README.md`.
- MCP server package skeleton under `src/mcp_lab/`.