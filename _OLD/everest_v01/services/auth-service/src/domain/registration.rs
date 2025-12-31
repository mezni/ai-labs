// src/domain/registration.rs
use crate::core::{AppError, IdGenerator, errors::AppResult};
use crate::domain::value_objects::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistrationStatus {
    Pending,
    Verified,
    Expired,
    Cancelled,
}

impl RegistrationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for RegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for RegistrationStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "verified" => Ok(Self::Verified),
            "expired" => Ok(Self::Expired),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(AppError::Validation(format!(
                "Invalid registration status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistration {
    pub registration_id: String,
    pub email: Email,
    pub username: Username,
    pub name: PersonName,
    pub phone: PhoneNumber,
    pub verification_token: String,
    pub verification_code: Option<String>,
    pub status: RegistrationStatus,
    pub keycloak_id: Option<String>,
    pub user_id: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl UserRegistration {
    const EXPIRATION_HOURS: i64 = 24;
    const TOKEN_LENGTH: usize = 32;
    const CODE_LENGTH: usize = 6;

    /// Create a new registration
    pub fn new(
        email: Email,
        username: Username,
        name: PersonName,
        phone: PhoneNumber,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::hours(Self::EXPIRATION_HOURS);

        Self {
            registration_id: IdGenerator::registration_id(),
            email,
            username,
            name,
            phone,
            verification_token: Self::generate_token(),
            verification_code: Some(Self::generate_code()),
            status: RegistrationStatus::Pending,
            keycloak_id: None,
            user_id: None,
            expires_at,
            verified_at: None,
            created_at: now,
            ip_address,
            user_agent,
        }
    }

    /// Generate a random verification token
    fn generate_token() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                  abcdefghijklmnopqrstuvwxyz\
                                  0123456789";
        let mut rng = rand::thread_rng();
        (0..Self::TOKEN_LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Generate a numeric verification code
    fn generate_code() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let code: u32 = rng.gen_range(100000..999999);
        code.to_string()
    }

    /// Mark as verified
    pub fn verify(&mut self, keycloak_id: String, user_id: String) -> AppResult<()> {
        if self.status != RegistrationStatus::Pending {
            return Err(AppError::BadRequest(format!(
                "Cannot verify registration with status: {}",
                self.status
            )));
        }

        if self.is_expired() {
            self.status = RegistrationStatus::Expired;
            return Err(AppError::BadRequest("Registration has expired".to_string()));
        }

        self.status = RegistrationStatus::Verified;
        self.keycloak_id = Some(keycloak_id);
        self.user_id = Some(user_id);
        self.verified_at = Some(Utc::now());

        Ok(())
    }

    /// Cancel registration
    pub fn cancel(&mut self) -> AppResult<()> {
        if self.status == RegistrationStatus::Verified {
            return Err(AppError::BadRequest(
                "Cannot cancel verified registration".to_string(),
            ));
        }

        self.status = RegistrationStatus::Cancelled;
        Ok(())
    }

    /// Check if registration is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if registration is pending
    pub fn is_pending(&self) -> bool {
        self.status == RegistrationStatus::Pending && !self.is_expired()
    }

    /// Check if registration is verified
    pub fn is_verified(&self) -> bool {
        self.status == RegistrationStatus::Verified
    }

    /// Validate verification token
    pub fn validate_token(&self, token: &str) -> bool {
        self.verification_token == token && self.is_pending()
    }

    /// Validate verification code
    pub fn validate_code(&self, code: &str) -> bool {
        if let Some(ref expected_code) = self.verification_code {
            expected_code == code && self.is_pending()
        } else {
            false
        }
    }

    /// Get time remaining until expiration
    pub fn time_until_expiration(&self) -> Option<Duration> {
        let now = Utc::now();
        if now < self.expires_at {
            Some(self.expires_at - now)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_registration() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let name = PersonName::new(Some("Test".to_string()), Some("User".to_string())).unwrap();
        let phone = PhoneNumber::new(None).unwrap();

        let reg = UserRegistration::new(email, username, name, phone, None, None);

        assert_eq!(reg.status, RegistrationStatus::Pending);
        assert!(reg.is_pending());
        assert!(!reg.is_expired());
        assert!(reg.verification_token.len() == UserRegistration::TOKEN_LENGTH);
        assert!(reg.verification_code.is_some());
    }

    #[test]
    fn test_verify_registration() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let name = PersonName::new(None, None).unwrap();
        let phone = PhoneNumber::new(None).unwrap();

        let mut reg = UserRegistration::new(email, username, name, phone, None, None);

        let result = reg.verify("keycloak-123".to_string(), "USR-123".to_string());
        assert!(result.is_ok());
        assert!(reg.is_verified());
        assert!(reg.verified_at.is_some());
        assert_eq!(reg.keycloak_id, Some("keycloak-123".to_string()));
        assert_eq!(reg.user_id, Some("USR-123".to_string()));
    }

    #[test]
    fn test_validate_token() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let name = PersonName::new(None, None).unwrap();
        let phone = PhoneNumber::new(None).unwrap();

        let reg = UserRegistration::new(email, username, name, phone, None, None);

        assert!(reg.validate_token(&reg.verification_token.clone()));
        assert!(!reg.validate_token("invalid-token"));
    }
}
