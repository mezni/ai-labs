# Changelog

All notable changes to the AI Ticket Triage Agent project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- Initial project structure
- README.md with project overview, objectives, scope, technology stack, architecture, roadmap, repository structure, development principles, development tracking, getting started, definition of done, and final objective
- docs/architecture.md with Mermaid CSS/architecture diagram and architecture layers table with important design decisions
- LOG.md with development tracking structure (Step 1: Initialize Tracking Structure)

### Changed
- None

### Deprecated
- None

### Removed
- None

### Fixed
- None

## 0.1.3 - 2026-10-12

### Added
- priority module with deterministic ticket urgency assessment
- assess_priority() function with ordered keyword rules (critical, high, medium, low)
- Input validation and reason generation
- 9 pytest tests covering all priority levels, rule precedence, and invalid input
- Ruff configuration for linting and formatting

### Changed
- pyproject.toml: Added pytest and ruff configuration
- LOG.md: Development tracking structure

### Fixed
- None

## 0.1.2 - 2026-10-12

### Added
- classifier module with rule-based ticket categorization
- Input validation and entity extraction
- Keyword-based category detection (order_status, billing, account, technical, general)
- Structured output with category, order_id, and needs_order_lookup fields
- 9 pytest tests covering normal cases, edge cases, and invalid input
- Ruff configuration for linting and formatting

### Changed
- pyproject.toml: Added pytest and ruff configuration
- LOG.md: Development tracking structure

### Fixed
- None

## 0.1.1 - 2026-10-12

### Added
- Initial project structure
- README.md with project overview, objectives, scope, technology stack, architecture, roadmap, repository structure, development principles, development tracking, getting started, definition of done, and final objective
- docs/architecture.md with Mermaid CSS/architecture diagram and architecture layers table with important design decisions
- LOG.md with development tracking structure (Step 1: Initialize Tracking Structure)