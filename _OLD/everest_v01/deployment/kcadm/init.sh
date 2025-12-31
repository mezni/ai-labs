#!/bin/bash
set -euo pipefail

########################################
# Configuration (env overridable)
########################################
KEYCLOAK_URL="${KEYCLOAK_URL:-http://keycloak:8080}"
ADMIN_USER="${KEYCLOAK_ADMIN_USER:-admin}"
ADMIN_PASS="${KEYCLOAK_ADMIN_PASSWORD:-password}"
REALM="${KEYCLOAK_REALM:-myrealm}"

FRONTEND_CLIENT_ID="${FRONTEND_CLIENT_ID:-auth-client}"
BACKEND_CLIENT_ID="${BACKEND_CLIENT_ID:-backend-admin}"
BACKEND_CLIENT_SECRET="${BACKEND_CLIENT_SECRET:-backend-admin-secret}"

ATTR_SCOPE="auth-attributes"

KCADM="/opt/keycloak/bin/kcadm.sh"
LOGIN_FLAGS="--server $KEYCLOAK_URL --realm master --user $ADMIN_USER --password $ADMIN_PASS"

########################################
# Helpers
########################################
log() { echo "[$(date +'%H:%M:%S')] $*"; }

kc() { $KCADM "$@" $LOGIN_FLAGS; }

get_client_uuid() {
  kc get clients -r "$REALM" -q clientId="$1" --fields id --format csv | tail -n1 | tr -d '"'
}

get_scope_uuid() {
  kc get client-scopes -r "$REALM" -q name="$1" --fields id --format csv | tail -n1 | tr -d '"'
}

get_user_uuid() {
  kc get users -r "$REALM" -q username="$1" --fields id --format csv | tail -n1 | tr -d '"'
}

########################################
# Wait for Keycloak
########################################
log "Waiting for Keycloak..."
for _ in {1..60}; do
  kc get realms/master >/dev/null 2>&1 && break
  sleep 5
done
log "Keycloak is ready"


########################################
# Realm
########################################
if ! kc get realms/$REALM >/dev/null 2>&1; then
  log "Creating realm $REALM"
  kc create realms \
    -s realm="$REALM" \
    -s enabled=true \
    -s attributes.userProfileEnabled=true 
else
  log "Realm $REALM already exists"
fi

# -------------------------
# Enable User Profile + Unmanaged Attributes
# -------------------------
log "Enabling unmanaged attributes"
kc update realms/"$REALM"/users/profile \
  -r "$REALM" \
  -f - <<EOF
{
  "unmanagedAttributePolicy": "ENABLED"
}
EOF

########################################
# Roles
########################################
log "Create roles"
for ROLE in admin user operator partner; do
  kc get roles -r "$REALM" -q name="$ROLE" | grep -q name || {
    kc create roles -r "$REALM" -s name="$ROLE"
    log "Created role: $ROLE"
  }
done


########################################
# Clients
########################################
log "Configuring clients"

# Frontend
log "Frontend"
kc get clients -r "$REALM" -q clientId="$FRONTEND_CLIENT_ID" | grep -q id || \
kc create clients -r "$REALM" \
  -s clientId="$FRONTEND_CLIENT_ID" \
  -s protocol=openid-connect \
  -s publicClient=true \
  -s standardFlowEnabled=true \
  -s directAccessGrantsEnabled=true \
  -s redirectUris='["http://localhost:8081/*"]' \
  -s webOrigins='["http://localhost:8081"]'

# Backend (service account)
log "Backend"
kc get clients -r "$REALM" -q clientId="$BACKEND_CLIENT_ID" | grep -q id || \
kc create clients -r "$REALM" \
  -s clientId="$BACKEND_CLIENT_ID" \
  -s protocol=openid-connect \
  -s publicClient=false \
  -s secret="$BACKEND_CLIENT_SECRET" \
  -s serviceAccountsEnabled=true




########################################
# Assign Scope to Frontend Client
########################################
FRONTEND_UUID=$(get_client_uuid "$FRONTEND_CLIENT_ID")
log "$FRONTEND_UUID" 


create_mapper() {
  local NAME="$1"
#  log "Creating protocol mapper '$NAME'"
#  if kc get clients/$FRONTEND_UUID/protocol-mappers/models -r "$REALM" -q name="$NAME" | grep -q id; then
#    log "Protocol mapper '$NAME' already exists"
#    return
#  fi

  log "Creating protocol mapper '$NAME'"
  kc create clients/$FRONTEND_UUID/protocol-mappers/models -r "$REALM" -f -
  log "Created protocol mapper '$NAME'"
}

ROLES="user,admin,partner,operator"

create_mapper filtered-roles <<EOF
{
  "name":"filtered-roles",
  "protocol":"openid-connect",
  "protocolMapper":"oidc-role-name-mapper",
  "config":{
    "claim.name":"roles",
    "multivalued":"true",
    "access.token.claim":"true",
    "id.token.claim":"true",
    "userinfo.token.claim":"true",
    "role":"$ROLES"
  }
}
EOF

for ATTR in network_id station_id; do
create_mapper "$ATTR" <<EOF
{
  "name":"$ATTR",
  "protocol":"openid-connect",
  "protocolMapper":"oidc-usermodel-attribute-mapper",
  "config":{
    "user.attribute":"$ATTR",
    "claim.name":"$ATTR",
    "access.token.claim":"true",
    "id.token.claim":"true",
    "userinfo.token.claim":"true"
  }
}
EOF
done