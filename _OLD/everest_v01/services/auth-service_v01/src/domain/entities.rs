use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: String,
    pub keycloak_id: String,
    pub email: String,
    pub source: UserSource,
    pub roles: Vec<Role>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserSource {
    Web,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Partner,
    Operator,
    User,
}

impl Role {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(Role::Admin),
            "partner" => Some(Role::Partner),
            "operator" => Some(Role::Operator),
            "user" => Some(Role::User),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::Partner => "partner",
            Role::Operator => "operator",
            Role::User => "user",
        }
    }
}

impl User {
    pub fn validate_internal_user(&self) -> Result<(), String> {
        if self.source != UserSource::Internal {
            return Err("Not an internal user".to_string());
        }

        for role in &self.roles {
            match role {
                Role::Partner => {
                    if self.network_id.is_none() {
                        return Err("Partner role requires network_id".to_string());
                    }
                }
                Role::Operator => {
                    if self.network_id.is_none() || self.station_id.is_none() {
                        return Err("Operator role requires network_id and station_id".to_string());
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditLog {
    pub id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}