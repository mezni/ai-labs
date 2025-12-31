#!/bin/bash

set -e

echo "Setting up Keycloak for Auth Service..."
echo "========================================"

# Configuration
KEYCLOAK_URL="http://localhost:5080"
ADMIN_USER="admin"
ADMIN_PASSWORD="password"
REALM_NAME="myrealm"
BACKEND_CLIENT_ID="backend-admin"
AUTH_CLIENT_ID="auth-client"

# Get admin token
echo "1. Authenticating with Keycloak admin..."
ADMIN_TOKEN=$(curl -s -X POST \
  "${KEYCLOAK_URL}/realms/master/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=${ADMIN_USER}" \
  -d "password=${ADMIN_PASSWORD}" \
  -d "grant_type=password" \
  -d "client_id=admin-cli" | jq -r '.access_token')

if [ "$ADMIN_TOKEN" = "null" ] || [ -z "$ADMIN_TOKEN" ]; then
    echo "❌ Failed to get admin token. Is Keycloak running?"
    exit 1
fi
echo "✅ Admin authenticated"

# Create realm
echo ""
echo "2. Creating realm '${REALM_NAME}'..."
curl -s -X POST \
  "${KEYCLOAK_URL}/admin/realms" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"realm\": \"${REALM_NAME}\",
    \"enabled\": true,
    \"registrationAllowed\": false,
    \"registrationEmailAsUsername\": true,
    \"verifyEmail\": true,
    \"loginWithEmailAllowed\": true
  }" > /dev/null 2>&1

echo "✅ Realm created (or already exists)"

# Create backend-admin client (service account)
echo ""
echo "3. Creating backend-admin client (service account)..."
curl -s -X POST \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"clientId\": \"${BACKEND_CLIENT_ID}\",
    \"enabled\": true,
    \"protocol\": \"openid-connect\",
    \"publicClient\": false,
    \"serviceAccountsEnabled\": true,
    \"directAccessGrantsEnabled\": false,
    \"standardFlowEnabled\": false,
    \"clientAuthenticatorType\": \"client-secret\",
    \"secret\": \"backend-admin-secret\"
  }" > /dev/null 2>&1

echo "✅ Backend client created"

# Get backend client UUID
BACKEND_CLIENT_UUID=$(curl -s -X GET \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients?clientId=${BACKEND_CLIENT_ID}" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.[0].id')

# Get service account user ID
SERVICE_ACCOUNT_ID=$(curl -s -X GET \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients/${BACKEND_CLIENT_UUID}/service-account-user" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.id')

# Get realm-management client UUID
REALM_MGMT_UUID=$(curl -s -X GET \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients?clientId=realm-management" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.[0].id')

# Get roles
MANAGE_USERS_ROLE=$(curl -s -X GET \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients/${REALM_MGMT_UUID}/roles/manage-users" \
  -H "Authorization: Bearer $ADMIN_TOKEN")

VIEW_USERS_ROLE=$(curl -s -X GET \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients/${REALM_MGMT_UUID}/roles/view-users" \
  -H "Authorization: Bearer $ADMIN_TOKEN")

# Assign roles to service account
echo ""
echo "4. Assigning admin roles to backend client..."
curl -s -X POST \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/users/${SERVICE_ACCOUNT_ID}/role-mappings/clients/${REALM_MGMT_UUID}" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d "[${MANAGE_USERS_ROLE}, ${VIEW_USERS_ROLE}]" > /dev/null 2>&1

echo "✅ Admin roles assigned"

# Create auth-client (public client)
echo ""
echo "5. Creating auth-client (public client)..."
curl -s -X POST \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients" \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"clientId\": \"${AUTH_CLIENT_ID}\",
    \"enabled\": true,
    \"protocol\": \"openid-connect\",
    \"publicClient\": true,
    \"standardFlowEnabled\": true,
    \"directAccessGrantsEnabled\": true,
    \"redirectUris\": [\"http://localhost:3000/*\"],
    \"webOrigins\": [\"http://localhost:3000\"]
  }" > /dev/null 2>&1

echo "✅ Auth client created"

# Get the actual client secret
echo ""
echo "6. Retrieving client secret..."
CLIENT_SECRET=$(curl -s -X GET \
  "${KEYCLOAK_URL}/admin/realms/${REALM_NAME}/clients/${BACKEND_CLIENT_UUID}/client-secret" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq -r '.value')

echo ""
echo "========================================"
echo "✅ Keycloak setup complete!"
echo "========================================"
echo ""
echo "Update your .env file with:"
echo ""
echo "KEYCLOAK_URL=http://localhost:5080"
echo "KEYCLOAK_REALM=${REALM_NAME}"
echo "KEYCLOAK_AUTH_CLIENT_ID=${AUTH_CLIENT_ID}"
echo "KEYCLOAK_BACKEND_CLIENT_ID=${BACKEND_CLIENT_ID}"
echo "KEYCLOAK_BACKEND_CLIENT_SECRET=${CLIENT_SECRET}"
echo ""
echo "Then test with: bash test_keycloak_setup.sh"