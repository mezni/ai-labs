#!/usr/bin/env bash

set -e

echo "-------------------------------------------"
echo " Running SQLx Prepare"
echo "-------------------------------------------"

# Load .env if exists
if [ -f .env ]; then
    echo "Loading .env..."
    export $(grep -v '^#' .env | xargs)
fi

if [ -z "$DATABASE_URL" ]; then
    echo "❌ ERROR: DATABASE_URL is not set!"
    echo "Make sure it exists in your .env"
    exit 1
fi

echo "Using DATABASE_URL=$DATABASE_URL"
echo "Preparing SQLx offline cache..."

# Ensure .sqlx folder exists
mkdir -p .sqlx

# Generate SQLx cache
cargo sqlx prepare --database-url "$DATABASE_URL"

echo "-------------------------------------------"
echo " SQLx prepare completed successfully!"
echo "-------------------------------------------"
echo "You can now run:  docker compose build"
