// src/domain/user.rs
use crate::core::{AppError, IdGenerator, errors::AppResult};
use crate::domain::value_objects::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub keycloak_id: String,
    pub email: Email,
    pub username: Username,
    pub name: PersonName,
    pub phone: PhoneNumber,
    pub photo: Option<String>,
    pub is_verified: bool,
    pub role: UserRole,
    pub network_id: String,
    pub station_id: String,
    pub source: UserSource,
    pub is_active: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl User {
    /// Create a new external user (self-registration)
    pub fn new_external(
        keycloak_id: String,
        email: Email,
        username: Username,
        name: PersonName,
        phone: PhoneNumber,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id: IdGenerator::user_id(),
            keycloak_id,
            email,
            username,
            name,
            phone,
            photo: None,
            is_verified: false,
            role: UserRole::user(),
            network_id: String::new(),
            station_id: String::new(),
            source: UserSource::web(),
            is_active: true,
            deleted_at: None,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }

    /// Create a new internal user (admin-created)
    pub fn new_internal(
        keycloak_id: String,
        email: Email,
        username: Username,
        name: PersonName,
        phone: PhoneNumber,
        role: UserRole,
        network_id: Option<NetworkId>,
        station_id: Option<StationId>,
        created_by: String,
    ) -> AppResult<Self> {
        // Validate role constraints
        if role.requires_network_id() && network_id.is_none() {
            return Err(AppError::Validation(format!(
                "Role '{}' requires network_id",
                role.as_str()
            )));
        }

        if role.requires_station_id() && station_id.is_none() {
            return Err(AppError::Validation(format!(
                "Role '{}' requires station_id",
                role.as_str()
            )));
        }

        let now = Utc::now();
        Ok(Self {
            user_id: IdGenerator::user_id(),
            keycloak_id,
            email,
            username,
            name,
            phone,
            photo: None,
            is_verified: true, // Internal users are pre-verified
            role,
            network_id: network_id.map(|n| n.into_inner()).unwrap_or_default(),
            station_id: station_id.map(|s| s.into_inner()).unwrap_or_default(),
            source: UserSource::internal(),
            is_active: true,
            deleted_at: None,
            created_at: now,
            updated_at: now,
            created_by: Some(created_by),
            updated_by: None,
        })
    }

    /// Update user profile (self-update, no email/password changes)
    pub fn update_profile(
        &mut self,
        name: PersonName,
        phone: PhoneNumber,
        photo: Option<String>,
    ) -> AppResult<()> {
        self.name = name;
        self.phone = phone;
        self.photo = photo;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Admin update user
    pub fn admin_update(
        &mut self,
        email: Option<Email>,
        username: Option<Username>,
        name: Option<PersonName>,
        phone: Option<PhoneNumber>,
        photo: Option<String>,
        role: Option<UserRole>,
        network_id: Option<NetworkId>,
        station_id: Option<StationId>,
        updated_by: String,
    ) -> AppResult<()> {
        if let Some(e) = email {
            self.email = e;
        }

        if let Some(u) = username {
            self.username = u;
        }

        if let Some(n) = name {
            self.name = n;
        }

        if let Some(p) = phone {
            self.phone = p;
        }

        if photo.is_some() {
            self.photo = photo;
        }

        if let Some(r) = role {
            // Validate role constraints
            if r.requires_network_id() && self.network_id.is_empty() && network_id.is_none() {
                return Err(AppError::Validation(format!(
                    "Role '{}' requires network_id",
                    r.as_str()
                )));
            }

            if r.requires_station_id() && self.station_id.is_empty() && station_id.is_none() {
                return Err(AppError::Validation(format!(
                    "Role '{}' requires station_id",
                    r.as_str()
                )));
            }

            self.role = r;
        }

        if let Some(n) = network_id {
            self.network_id = n.into_inner();
        }

        if let Some(s) = station_id {
            self.station_id = s.into_inner();
        }

        self.updated_at = Utc::now();
        self.updated_by = Some(updated_by);

        Ok(())
    }

    /// Soft delete user
    pub fn soft_delete(&mut self, deleted_by: String) -> AppResult<()> {
        if self.deleted_at.is_some() {
            return Err(AppError::BadRequest("User is already deleted".to_string()));
        }

        self.is_active = false;
        self.deleted_at = Some(Utc::now());
        self.updated_by = Some(deleted_by);
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Verify email
    pub fn verify_email(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }

    /// Check if user is deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Check if user is internal
    pub fn is_internal(&self) -> bool {
        self.source.is_internal()
    }

    /// Check if user is external
    pub fn is_external(&self) -> bool {
        !self.is_internal()
    }

    /// Get display name
    pub fn display_name(&self) -> String {
        self.name
            .full_name()
            .unwrap_or_else(|| self.username.as_str().to_string())
    }

    /// Validate business invariants
    pub fn validate(&self) -> AppResult<()> {
        // Role constraints
        if self.role.requires_network_id() && self.network_id.is_empty() {
            return Err(AppError::Validation(format!(
                "Role '{}' requires network_id",
                self.role.as_str()
            )));
        }

        if self.role.requires_station_id() && self.station_id.is_empty() {
            return Err(AppError::Validation(format!(
                "Role '{}' requires station_id",
                self.role.as_str()
            )));
        }

        // Source-role constraints
        if self.source.is_internal() && !self.role.is_internal() {
            return Err(AppError::Validation(
                "Internal users cannot have 'user' role".to_string(),
            ));
        }

        if !self.source.is_internal() && self.role.is_internal() {
            return Err(AppError::Validation(
                "External users cannot have internal roles".to_string(),
            ));
        }

        // Deleted state constraints
        if self.is_deleted() && self.is_active {
            return Err(AppError::Validation(
                "Deleted users cannot be active".to_string(),
            ));
        }

        Ok(())
    }
}

// Builder pattern for creating users
pub struct UserBuilder {
    keycloak_id: String,
    email: Email,
    username: Username,
    name: PersonName,
    phone: PhoneNumber,
    photo: Option<String>,
    role: UserRole,
    network_id: Option<NetworkId>,
    station_id: Option<StationId>,
    source: UserSource,
    created_by: Option<String>,
}

impl UserBuilder {
    pub fn new(keycloak_id: String, email: Email, username: Username) -> Self {
        Self {
            keycloak_id,
            email,
            username,
            name: PersonName::new(None, None).unwrap(),
            phone: PhoneNumber::new(None).unwrap(),
            photo: None,
            role: UserRole::user(),
            network_id: None,
            station_id: None,
            source: UserSource::web(),
            created_by: None,
        }
    }

    pub fn name(
        mut self,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> AppResult<Self> {
        self.name = PersonName::new(first_name, last_name)?;
        Ok(self)
    }

    pub fn phone(mut self, phone: Option<String>) -> AppResult<Self> {
        self.phone = PhoneNumber::new(phone)?;
        Ok(self)
    }

    pub fn photo(mut self, photo: Option<String>) -> Self {
        self.photo = photo;
        self
    }

    pub fn role(mut self, role: UserRole) -> Self {
        self.role = role;
        self
    }

    pub fn network_id(mut self, network_id: Option<NetworkId>) -> Self {
        self.network_id = network_id;
        self
    }

    pub fn station_id(mut self, station_id: Option<StationId>) -> Self {
        self.station_id = station_id;
        self
    }

    pub fn source(mut self, source: UserSource) -> Self {
        self.source = source;
        self
    }

    pub fn created_by(mut self, created_by: String) -> Self {
        self.created_by = Some(created_by);
        self
    }

    pub fn build(self) -> AppResult<User> {
        let now = Utc::now();
        let user = User {
            user_id: IdGenerator::user_id(),
            keycloak_id: self.keycloak_id,
            email: self.email,
            username: self.username,
            name: self.name,
            phone: self.phone,
            photo: self.photo,
            is_verified: self.source.is_internal(),
            role: self.role,
            network_id: self.network_id.map(|n| n.into_inner()).unwrap_or_default(),
            station_id: self.station_id.map(|s| s.into_inner()).unwrap_or_default(),
            source: self.source,
            is_active: true,
            deleted_at: None,
            created_at: now,
            updated_at: now,
            created_by: self.created_by,
            updated_by: None,
        };

        user.validate()?;
        Ok(user)
    }
}
