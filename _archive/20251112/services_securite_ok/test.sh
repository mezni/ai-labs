#!/bin/bash

# Set the shared secret
export JWT_SECRET="shared-secret-key-12345"

echo "=== Testing Distributed Authentication ==="

# Step 1: Get token from Auth Service
echo -e "\n1. Getting token from Auth Service (port 5100)..."
TOKEN_RESPONSE=$(curl -s -X POST http://localhost:5100/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}')

echo "Auth Service Response: $TOKEN_RESPONSE"

# Extract token
TOKEN=$(echo $TOKEN_RESPONSE | grep -o '"token":"[^"]*' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo "ERROR: Failed to get token from Auth Service"
    exit 1
fi

echo "Token obtained: ${TOKEN:0:50}..."

# Step 2: Use token with Network Service
echo -e "\n2. Creating network with Network Service (port 8080)..."
NETWORK_RESPONSE=$(curl -s -X POST http://localhost:8080/api/v1/networks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Test Network",
    "type": "company",
    "contact_email": "test@company.com"
  }')

echo "Network Service Response: $NETWORK_RESPONSE"

# Step 3: Verify network was created
echo -e "\n3. Verifying network creation..."
curl -s -X GET http://localhost:8080/api/v1/networks | jq '.'

echo -e "\n=== Test Complete ==="