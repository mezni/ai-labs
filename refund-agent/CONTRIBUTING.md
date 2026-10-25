# Contributing to Refund Agent 🦀

Thank you for your interest in contributing to **Refund Agent**, a Rust-based project exploring reliable AI agent engineering.

This project aims to demonstrate how LLM-powered reasoning can work alongside deterministic business rules, explicit error handling, automated evaluation, and human oversight.

Contributions that improve correctness, reliability, security, documentation, and testability are welcome.

## 📋 Table of Contents

- [Project Goals](#-project-goals)
- [Getting Started](#-getting-started)
- [Development Workflow](#-development-workflow)
- [Code Quality Standards](#-code-quality-standards)
- [Testing Requirements](#-testing-requirements)
- [AI Agent Engineering Principles](#-ai-agent-engineering-principles)
- [Commit Convention](#-commit-convention)
- [Pull Request Guidelines](#-pull-request-guidelines)
- [Reporting Issues](#-reporting-issues)
- [License](#-license)

## 🎯 Project Goals

Contributions should support one or more of the following goals:

- Build a maintainable refund agent in Rust.
- Keep business rules independent of LLM behavior.
- Improve error handling and resilience.
- Validate LLM outputs before using them.
- Protect sensitive operations with authorization and human approval.
- Establish reproducible evaluations and automated tests.
- Prepare the application for secure production deployment.

## 🚀 Getting Started

### Prerequisites

Install the following tools:

- [Rust stable toolchain](https://www.rust-lang.org/tools/install)
- Cargo, included with Rust
- Git

Install the required Rust components:

```bash
rustup component add rustfmt clippy
```

### Clone the Repository

```bash
git clone <YOUR_REPOSITORY_URL>
cd refund-agent
```

Replace `<YOUR_REPOSITORY_URL>` with the actual Git repository URL.

### Verify the Environment

```bash
rustc --version
cargo --version
cargo test
```

Run the application:

```bash
cargo run
```

## 🔄 Development Workflow

Follow this workflow for each contribution.

### 1. Create a Branch

```bash
git switch -c feature/short-description
```

Use an appropriate prefix:

- `feature/` — new functionality
- `fix/` — bug fixes
- `test/` — test improvements
- `docs/` — documentation changes
- `refactor/` — code restructuring without intended behavior changes

### 2. Implement a Focused Change

Keep changes small and focused on one problem.

Before adding a dependency, consider whether the Rust standard library or an existing project dependency already provides the required functionality.

### 3. Run Quality Checks

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All checks should pass before submitting a pull request.

### 4. Update Documentation

Update the relevant documentation when a change affects:

- Public interfaces
- Business rules
- Architecture
- Configuration
- Dependencies
- Evaluation criteria
- Operational behavior

Keep `README.md` and `LOG.md` consistent with the implemented state of the project.

### 5. Commit Your Changes

```bash
git add .
git commit -m "refund-agent: describe the change"
```

Use the commit convention described below.

## 🦀 Code Quality Standards

### Rust Style

- Follow standard Rust naming conventions.
- Use `snake_case` for functions, modules, and variables.
- Use `PascalCase` for structs, enums, and traits.
- Prefer clear, descriptive names.
- Use `cargo fmt` to format code.
- Resolve Clippy warnings rather than suppressing them without justification.

### Error Handling

- Use `Result<T, E>` for operations that can fail.
- Avoid silently discarding errors.
- Avoid unnecessary panics in application code.
- Provide useful error context at system boundaries.
- Distinguish business-rule rejections from technical failures.

### Type Safety

- Represent meaningful domain concepts with structs and enums.
- Avoid loosely structured data when a typed model is appropriate.
- Use integer monetary units or a suitable money type rather than floating-point arithmetic for refund amounts.
- Validate untrusted input at application boundaries.

### Dependency Management

- Prefer established, maintained dependencies.
- Add only dependencies needed by the implementation.
- Explain significant new dependencies in the pull request.
- Commit `Cargo.lock` for this application.

## 🧪 Testing Requirements

Tests should demonstrate that the change behaves correctly and does not introduce regressions.

### Unit Tests

Use unit tests for isolated business logic, including:

- Refund eligibility
- Refund limits
- Order status validation
- Invalid requests
- Decision outcomes
- Error handling

Example:

```rust
#[test]
fn refund_amount_is_stored_in_cents() {
    let amount_cents: u64 = 4_999;

    assert_eq!(amount_cents, 4_999);
}
```

This is an illustrative example; prefer testing actual domain behavior as the implementation develops.

### Integration Tests

Use integration tests for interactions between components, such as:

- Refund request processing
- Order lookup and ownership validation
- Workflow orchestration
- Approval and escalation paths
- Service failures and retries

### Test Quality

Tests should be deterministic and independent of external services whenever possible.

Use mocks or fakes for external APIs and payment services. Never execute real refunds during automated tests.

## 🛡️ AI Agent Engineering Principles

Changes to AI-related functionality should follow these principles.

### 1. Keep Policies Deterministic

The LLM may interpret customer language, but refund eligibility must be evaluated by explicit business rules.

### 2. Validate Model Output

Treat LLM output as untrusted input. Validate its structure, values, identifiers, and requested actions before using it.

### 3. Restrict Tool Permissions

Only expose tools required for the task. Validate tool arguments and enforce authorization independently of model-generated instructions.

### 4. Protect Sensitive Actions

Refund execution must follow the configured approval and authorization policy. Ambiguous or exceptional cases should be escalated when required.

### 5. Evaluate Changes

Changes to prompts, models, or intent extraction should be assessed against representative evaluation cases.

Avoid claiming improved accuracy without supporting evaluation results.

### 6. Protect Secrets and Customer Data

- Never commit API keys, passwords, or production credentials.
- Use environment variables or an appropriate secrets manager.
- Avoid logging unnecessary personal or payment information.
- Use synthetic customer and order data in examples and tests.

## 📝 Commit Convention

Use the following format:

```text
refund-agent: <short description>
```

Examples:

```text
refund-agent: add refund domain errors
refund-agent: implement deterministic refund policy
refund-agent: add workflow integration tests
refund-agent: validate structured LLM output
refund-agent: document retry behavior
```

Each commit should describe one meaningful change whenever practical.

## 🔍 Pull Request Guidelines

Before opening a pull request, verify that:

- [ ] The change addresses a clear problem or requirement.
- [ ] The code follows Rust conventions.
- [ ] Formatting and linting pass.
- [ ] Relevant tests have been added or updated.
- [ ] All tests pass.
- [ ] Documentation is updated where necessary.
- [ ] No secrets or sensitive customer data are included.
- [ ] Business rules and authorization checks remain enforced.
- [ ] New dependencies and important trade-offs are explained.

### Pull Request Description

Include the following information:

**Problem:** What issue does this change solve?

**Solution:** What was implemented?

**Testing:** Which commands and tests were run?

**Trade-offs:** What alternatives were considered?

**Risks:** Could the change affect refund decisions, security, or reliability?

Keep pull requests focused and provide enough context for reviewers to understand the implementation.

## 🐛 Reporting Issues

When reporting an issue, include:

- A clear description of the problem
- Steps to reproduce it
- Expected behavior
- Actual behavior
- Relevant error messages or logs
- Rust and Cargo versions when relevant

Remove secrets and sensitive customer information before sharing logs.

## 📄 License

By contributing to this project, you agree that your contributions will be distributed under the project's [MIT License](LICENSE).

---

Thank you for helping improve Refund Agent.

**Build carefully. Test thoroughly. Keep AI decisions accountable.** 🦀