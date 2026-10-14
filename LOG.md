Step 0: Init project
Goal	A reproducible repo skeleton that anyone can clone and run
Problem	No structure, no tooling, hard for others to run
Solution (files)	pyproject.toml, .python-version, src/, tests/test_smoke.py, .env.example, .gitignore, README.md stub, .github/workflows/ci.yml, ruff + pytest config
Trade-off	uv vs poetry: uv is faster and simpler, with one lockfile and no packaging ceremony. Code sits directly under src/ with no intermediate package, so I set package = false and pythonpath = ["src"] instead of building an installable package. You give up pip install . for now.
Example	uv run pytest → 1 passed