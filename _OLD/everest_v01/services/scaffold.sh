#!/usr/bin/env bash

set -e

SERVICE_NAME=auth-service

echo "📁 Creating Auth Service project: $SERVICE_NAME"

# Create Rust binary project
cargo new --bin $SERVICE_NAME
cd $SERVICE_NAME

# =========================
# Add dependencies
# =========================
cargo add actix-web
cargo add actix-cors
cargo add sqlx --features runtime-tokio-rustls,postgres,chrono,uuid
cargo add tokio --features full
cargo add tracing
cargo add tracing-subscriber
cargo add anyhow
cargo add thiserror
cargo add dotenvy
cargo add utoipa
cargo add utoipa-swagger-ui
cargo add nanoid
cargo add serde --features derive
cargo add serde_json
cargo add reqwest --features json,rustls-tls

# =========================
# Remove default main.rs (we will control layout)
# =========================
rm src/main.rs

# =========================
# Root files
# =========================
touch src/main.rs
touch src/lib.rs

# =========================
# Core
# =========================
mkdir -p src/core
touch src/core/mod.rs
touch src/core/config.rs
touch src/core/logging.rs
touch src/core/errors.rs
touch src/core/database.rs
touch src/core/jwt.rs
touch src/core/id_generator.rs
touch src/core/constants.rs
touch src/core/middleware.rs

# =========================
# Domain
# =========================
mkdir -p src/domain
touch src/domain/mod.rs
touch src/domain/entities.rs
touch src/domain/value_objects.rs
touch src/domain/repositories.rs
touch src/domain/events.rs

# =========================
# Application
# =========================
mkdir -p src/application
touch src/application/mod.rs
touch src/application/dtos.rs
touch src/application/commands.rs
touch src/application/queries.rs
touch src/application/services.rs

# =========================
# Infrastructure
# =========================
mkdir -p src/infrastructure
touch src/infrastructure/mod.rs
touch src/infrastructure/persistence.rs
touch src/infrastructure/keycloak.rs
touch src/infrastructure/cache.rs

# =========================
# Interfaces
# =========================
mkdir -p src/interfaces
touch src/interfaces/mod.rs
touch src/interfaces/handlers.rs
touch src/interfaces/routes.rs
touch src/interfaces/openapi.rs

echo "✅ Auth Service project structure created successfully."
