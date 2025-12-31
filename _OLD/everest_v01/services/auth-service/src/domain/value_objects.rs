// src/domain/value_objects.rs
use crate::core::{AppError, constants::*, errors::AppResult};
use serde::{Deserialize, Serialize};
use std::fmt;
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> AppResult<Self> {
        let email = email.trim().to_lowercase();

        if email.is_empty() {
            return Err(AppError::Validation("Email cannot be empty".to_string()));
        }

        if email.len() > MAX_EMAIL_LENGTH {
            return Err(AppError::Validation(format!(
                "Email cannot exceed {} characters",
                MAX_EMAIL_LENGTH
            )));
        }

        if !email.validate_email() {
            return Err(AppError::Validation("Invalid email format".to_string()));
        }

        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> AppResult<Self> {
        let username = username.trim().to_string();

        if username.is_empty() {
            return Err(AppError::Validation("Username cannot be empty".to_string()));
        }

        if username.len() > MAX_USERNAME_LENGTH {
            return Err(AppError::Validation(format!(
                "Username cannot exceed {} characters",
                MAX_USERNAME_LENGTH
            )));
        }

        // Username should be alphanumeric with underscores and hyphens
        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(AppError::Validation(
                "Username can only contain letters, numbers, underscores, and hyphens".to_string(),
            ));
        }

        Ok(Self(username))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRole(String);

impl UserRole {
    pub fn new(role: String) -> AppResult<Self> {
        if !is_valid_role(&role) {
            return Err(AppError::Validation(format!(
                "Invalid role: {}. Must be one of: {}",
                role,
                VALID_ROLES.join(", ")
            )));
        }
        Ok(Self(role))
    }

    pub fn user() -> Self {
        Self(ROLE_USER.to_string())
    }

    pub fn admin() -> Self {
        Self(ROLE_ADMIN.to_string())
    }

    pub fn partner() -> Self {
        Self(ROLE_PARTNER.to_string())
    }

    pub fn operator() -> Self {
        Self(ROLE_OPERATOR.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_internal(&self) -> bool {
        is_internal_role(&self.0)
    }

    pub fn requires_network_id(&self) -> bool {
        requires_network_id(&self.0)
    }

    pub fn requires_station_id(&self) -> bool {
        requires_station_id(&self.0)
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UserRole> for String {
    fn from(role: UserRole) -> Self {
        role.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSource(String);

impl UserSource {
    pub fn new(source: String) -> AppResult<Self> {
        if !is_valid_source(&source) {
            return Err(AppError::Validation(format!(
                "Invalid source: {}. Must be one of: {}",
                source,
                VALID_SOURCES.join(", ")
            )));
        }
        Ok(Self(source))
    }

    pub fn web() -> Self {
        Self(SOURCE_WEB.to_string())
    }

    pub fn internal() -> Self {
        Self(SOURCE_INTERNAL.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_internal(&self) -> bool {
        self.0 == SOURCE_INTERNAL
    }
}

impl fmt::Display for UserSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Add this for UserSource ---
impl From<UserSource> for String {
    fn from(source: UserSource) -> Self {
        source.0
    }
}

// --- Add this for NetworkId ---
impl From<NetworkId> for String {
    fn from(id: NetworkId) -> Self {
        id.0
    }
}

// --- Add this for StationId ---
impl From<StationId> for String {
    fn from(id: StationId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonName {
    first_name: Option<String>,
    last_name: Option<String>,
}

impl PersonName {
    pub fn new(first_name: Option<String>, last_name: Option<String>) -> AppResult<Self> {
        if let Some(ref fn_) = first_name {
            if fn_.len() > MAX_NAME_LENGTH {
                return Err(AppError::Validation(format!(
                    "First name cannot exceed {} characters",
                    MAX_NAME_LENGTH
                )));
            }
        }

        if let Some(ref ln) = last_name {
            if ln.len() > MAX_NAME_LENGTH {
                return Err(AppError::Validation(format!(
                    "Last name cannot exceed {} characters",
                    MAX_NAME_LENGTH
                )));
            }
        }

        Ok(Self {
            first_name: first_name
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
            last_name: last_name
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
        })
    }

    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    pub fn full_name(&self) -> Option<String> {
        match (&self.first_name, &self.last_name) {
            (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
            (Some(f), None) => Some(f.clone()),
            (None, Some(l)) => Some(l.clone()),
            (None, None) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber(Option<String>);

impl PhoneNumber {
    pub fn new(phone: Option<String>) -> AppResult<Self> {
        if let Some(ref p) = phone {
            let p = p.trim();
            if p.is_empty() {
                return Ok(Self(None));
            }

            if p.len() > MAX_PHONE_LENGTH {
                return Err(AppError::Validation(format!(
                    "Phone number cannot exceed {} characters",
                    MAX_PHONE_LENGTH
                )));
            }

            // Basic validation: starts with + or digit, contains only digits, spaces, hyphens, parentheses
            if !p
                .chars()
                .next()
                .map_or(false, |c| c == '+' || c.is_ascii_digit())
            {
                return Err(AppError::Validation(
                    "Phone number must start with + or a digit".to_string(),
                ));
            }

            if !p.chars().all(|c| c.is_ascii_digit() || " -+()".contains(c)) {
                return Err(AppError::Validation(
                    "Phone number contains invalid characters".to_string(),
                ));
            }
        }

        Ok(Self(
            phone
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
        ))
    }

    pub fn as_str(&self) -> Option<&str> {
        self.0.as_deref()
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkId(String);

impl NetworkId {
    pub fn new(id: String) -> AppResult<Self> {
        if id.is_empty() {
            return Err(AppError::Validation(
                "Network ID cannot be empty".to_string(),
            ));
        }

        if !crate::core::IdGenerator::is_valid_format(&id, PREFIX_NETWORK) {
            return Err(AppError::Validation(format!(
                "Invalid network ID format: {}",
                id
            )));
        }

        Ok(Self(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for NetworkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StationId(String);

impl StationId {
    pub fn new(id: String) -> AppResult<Self> {
        if id.is_empty() {
            return Err(AppError::Validation(
                "Station ID cannot be empty".to_string(),
            ));
        }

        if !crate::core::IdGenerator::is_valid_format(&id, PREFIX_STATION) {
            return Err(AppError::Validation(format!(
                "Invalid station ID format: {}",
                id
            )));
        }

        Ok(Self(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for StationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
