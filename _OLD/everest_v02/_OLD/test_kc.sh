#!/bin/bash

# Load environment variables
source .env

echo "Testing Keycloak Configuration..."
echo "=================================="
echo ""

# Test 1: Get Access Token
echo "1. Testing Backend Client Authentication..."
TOKEN_RESPONSE=$(curl -s -X POST \
  "${KEYCLOAK_URL}/realms/${KEYCLOAK_REALM}/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=client_credentials" \
  -d "client_id=${KEYCLOAK_BACKEND_CLIENT_ID}" \
  -d "client_secret=${KEYCLOAK_BACKEND_CLIENT_SECRET}")

if echo "$TOKEN_RESPONSE" | grep -q "access_token"; then
    echo "✅ SUCCESS: Backend client authenticated"
    ACCESS_TOKEN=$(echo "$TOKEN_RESPONSE" | jq -r '.access_token')
    echo "   Token obtained successfully"
else
    echo "❌ FAILED: Cannot authenticate backend client"
    echo "   Response: $TOKEN_RESPONSE"
    exit 1
fi

echo ""

# Test 2: Check Admin Permissions
echo "2. Testing Admin Permissions (List Users)..."
USERS_RESPONSE=$(curl -s -X GET \
  "${KEYCLOAK_URL}/admin/realms/${KEYCLOAK_REALM}/users?max=1" \
  -H "Authorization: Bearer $ACCESS_TOKEN")

if echo "$USERS_RESPONSE" | grep -q "\["; then
    echo "✅ SUCCESS: Can access admin API"
    echo "   Admin permissions verified"
else
    echo "❌ FAILED: Cannot access admin API"
    echo "   Response: $USERS_RESPONSE"
    echo ""
    echo "   Make sure backend-admin client has these roles:"
    echo "   - realm-management: manage-users"
    echo "   - realm-management: view-users"
    exit 1
fi

echo ""

# Test 3: Create Test User
echo "3. Testing User Creation..."
TEST_EMAIL="test-$(date +%s)@example.com"
CREATE_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
  "${KEYCLOAK_URL}/admin/realms/${KEYCLOAK_REALM}/users" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"testuser$(date +%s)\",
    \"email\": \"$TEST_EMAIL\",
    \"enabled\": false,
    \"emailVerified\": false
  }")

HTTP_CODE=$(echo "$CREATE_RESPONSE" | tail -n1)

if [ "$HTTP_CODE" = "201" ]; then
    echo "✅ SUCCESS: Can create users"
    echo "   Test user created: $TEST_EMAIL"
else
    echo "❌ FAILED: Cannot create users (HTTP $HTTP_CODE)"
    echo "   Response: $CREATE_RESPONSE"
    exit 1
fi

echo ""
echo "=================================="
echo "✅ All Keycloak tests passed!"
echo "=================================="
echo ""
echo "Your configuration:"
echo "  Keycloak URL: ${KEYCLOAK_URL}"
echo "  Realm: ${KEYCLOAK_REALM}"
echo "  Backend Client: ${KEYCLOAK_BACKEND_CLIENT_ID}"
echo "  Auth Client: ${KEYCLOAK_AUTH_CLIENT_ID}"
echo ""
echo "You can now start the auth service with: make run"