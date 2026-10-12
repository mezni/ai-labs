# Development Log — AI Ticket Triage Agent

## Step 1: Initialize Tracking Structure

**Goal:** Set up a structured LOG.md to track agent development progress.

**Problem:** No existing tracking format for iterative problem-solving and documenting technical decisions.

**Solution:** Created LOG.md with four core fields: Component/Step, Goal, Problem/Friction, and Solution & Trade-offs.

**Trade-off:** Chose Markdown over a database or Notion DB for simplicity, Git version control, and easy review.

**Status:** Completed

## Step 2: Implement Ticket Classification

Component/Step: Ticket classification and structured extraction.

Goal: Convert plain-text support tickets into structured data for downstream agent components.

Problem/Friction: Ticket descriptions are unstructured. The application needs to identify the category, extract an order ID, and determine whether an order lookup is necessary.

Solution: Implemented classify_ticket() in src/ticket_triage/classifier.py. The function validates input, extracts numeric order IDs using regular expressions, classifies tickets using keyword rules, and returns a dictionary with category, order_id, and needs_order_lookup.

Trade-offs: Chose deterministic keyword-based rules over an LLM to make the initial implementation predictable, easy to debug, and inexpensive. The approach has limited semantic understanding and can misclassify ambiguous descriptions.

Tests: Added pytest coverage for order classification, missing order IDs, billing, account issues, general tickets, and invalid input. All 9 tests pass.

Quality Checks: uv run pytest -v, uv run ruff check ., and uv run ruff format --check .. All checks pass.

Status: Completed

## Step 3: Add Ticket Priority Scoring

Component/Step: Priority Engine.

Goal: Determine the urgency of support tickets using deterministic Python rules.

Problem/Friction: Ticket classification identifies the type of issue but does not determine how urgently it should be handled.

Solution: Implemented assess_priority() in src/ticket_triage/priority.py. The function checks ordered keyword rules and returns a priority (low, medium, high, or critical) with an explanation.

Trade-offs: Chose ordered keyword rules over machine learning or an LLM for simplicity, explainability, and reproducible results. The approach cannot reliably infer urgency from context and may miss unfamiliar wording. The initial rules require future refinement against real support policies.

Tests: Added pytest cases for each priority level, rule precedence, empty input, and invalid input. All 9 tests pass.

Quality Checks: uv run pytest -v, uv run ruff check ., and uv run ruff format --check .. All checks pass.

Status: Completed

Next Step: Implement response generation and integrate classification and priority into a unified ticket-processing workflow.