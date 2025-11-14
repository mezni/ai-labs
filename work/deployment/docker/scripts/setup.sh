#!/bin/bash

set -e

echo "Setting up Auth Service environment..."

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_ROOT="$(dirname "$(dirname "$DOCKER_DIR")")"

echo "Script directory: $SCRIPT_DIR"
echo "Docker directory: $DOCKER_DIR"
echo "Project root: $PROJECT_ROOT"

# Copy environment files if they don't exist
if [ ! -f "$DOCKER_DIR/.env" ]; then
    echo "Creating .env file from .env.example..."
    cp "$DOCKER_DIR/.env.example" "$DOCKER_DIR/.env"
    echo "Please edit $DOCKER_DIR/.env file with your actual values before running docker-compose"
else
    echo ".env file already exists at $DOCKER_DIR/.env"
fi

if [ ! -f "$PROJECT_ROOT/services/auth-service/.env" ]; then
    echo "Creating service .env file..."
    if [ -f "$PROJECT_ROOT/services/auth-service/.env.example" ]; then
        cp "$PROJECT_ROOT/services/auth-service/.env.example" "$PROJECT_ROOT/services/auth-service/.env"
    else
        echo "Warning: .env.example not found in service directory"
    fi
else
    echo "Service .env file already exists"
fi

# Generate a random JWT secret if not set
if [ -f "$DOCKER_DIR/.env" ] && grep -q "your-super-secret-production-jwt-key-change-this-in-production" "$DOCKER_DIR/.env"; then
    echo "Generating random JWT secret..."
    JWT_SECRET=$(openssl rand -base64 32 2>/dev/null || echo "fallback-jwt-secret-key-change-in-production")
    sed -i.bak "s|your-super-secret-production-jwt-key-change-this-in-production|$JWT_SECRET|g" "$DOCKER_DIR/.env"
    
    # Update service .env if it exists
    if [ -f "$PROJECT_ROOT/services/auth-service/.env" ]; then
        sed -i.bak "s|your-super-secret-production-jwt-key-change-this-in-production|$JWT_SECRET|g" "$PROJECT_ROOT/services/auth-service/.env"
    fi
    
    # Clean up backup files
    rm -f "$DOCKER_DIR/.env.bak" "$PROJECT_ROOT/services/auth-service/.env.bak" 2>/dev/null
    echo "JWT secret generated and updated in .env files"
fi

echo "Setup complete!"
echo "You may want to review the .env files before starting the services:"
echo "  - $DOCKER_DIR/.env"
echo "  - $PROJECT_ROOT/services/auth-service/.env"
echo ""
echo "To build and start the services:"
echo "  cd $DOCKER_DIR"
echo "  docker-compose build auth-service"
echo "  docker-compose up auth-service"