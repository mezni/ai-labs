Step 0: Init project
Goal	A reproducible repo skeleton that anyone can clone and run
Problem	No structure, no tooling, hard for others to run
Solution (files)	pyproject.toml, .python-version, src/, tests/test_smoke.py, .env.example, .gitignore, README.md stub, .github/workflows/ci.yml, ruff + pytest config
Trade-off	uv vs poetry: uv is faster and simpler, with one lockfile and no packaging ceremony. Code sits directly under src/ with no intermediate package, so I set package = false and pythonpath = ["src"] instead of building an installable package. You give up pip install . for now.
Example	uv run pytest → 1 passed


Step 1: Corpus and domain scope
	
Goal	A small, realistic telecom document set plus a manifest that describes every document
Problem	Without realistic docs, retrieval looks fake in a portfolio. Without metadata (version, plan type, effective dates), you can’t filter stale answers later (step 7).
Solution (files)	data/raw/{billing,roaming,plans,network}/*.md, data/manifest.json, data/README.md, tests/test_corpus.py, one-line fix in .gitignore
Trade-off	Synthetic docs (safe, controllable, no licensing) vs real operator docs (realistic, legal risk). Start with 5 short docs instead of 12: enough to test every later step, and you can grow the set. Markdown first (diffable in git). PDFs come in step 2 to exercise a second parser.
Example	make test → 5 passed