# bio-com

A modular monolith e-commerce platform built with Rust,
TypeScript, React, and PostgreSQL.

## Technology Stack

- Rust
- Axum
- PostgreSQL
- SQLx
- TypeScript
- React
- Vite

## Architecture

The backend follows Domain-Driven Design (DDD)
and Clean Architecture.

The frontend uses a feature-based architecture.

## Repository Structure

- apps/api: Rust REST API
- apps/web: React web application
- crates/domain: Domain model
- crates/application: Application use cases
- crates/infrastructure: Technical integrations
- crates/shared: Shared technical components
- tools/data-generator: Sample data generator
- migrations: Database migrations
- docs: Project documentation

## Getting Started

### Backend

```bash
cargo run -p bio-com-api
