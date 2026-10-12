# Autonomous AI Agent

An extensible AI agent built to execute multi-step workflows, interact with external tools, and maintain context across execution cycles.

## 📌 Architecture Overview

```text
[ User Prompt ] ──> [ Agent Core Loop ] ──> [ Memory / State ]
                           │
                           ├──> [ System Prompt / Constraints ]
                           └──> [ Tool Execution Layer ]
                                      ├── API / Web Search
                                      └── Custom Scripts
🛠️ Tooling & Stack
Package Manager: uv (fast Python project and dependency management)
Linter & Formatter: Ruff
Testing Framework: pytest
🚀 Setup & Local Development
1. Prerequisites
Install uv if you haven't already:

# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh

# Windows
powershell -c "irm https://astral.sh/uv/install.ps1 | iex"

2. Environment Setup
Install dependencies and sync the virtual environment automatically using uv:
uv sync

3. Environment Variables
Copy the template .env.example file and set your credentials:
cp .env.example .env
Add your API keys inside .env:
OPENAI_API_KEY=your_openai_key_here

🏃 Usage & Workflow Commands
Run the Agent
Run commands directly within the uv environment without activating virtualenvs manually:
uv run python main.py

Lint & Format (Ruff)
# Check for lint issues
uv run ruff check .

# Fix auto-fixable lint issues
uv run ruff check . --fix

# Format code
uv run ruff format .

Run Tests (pytest)
# Run all tests
uv run pytest

# Run with verbose output and print statements
uv run pytest -s -v

📝 Documenting Decisions & Progress
Log problems, goals, and resolutions in CHANGELOG.md whenever you modify agent architecture, prompts, or tool execution paths.