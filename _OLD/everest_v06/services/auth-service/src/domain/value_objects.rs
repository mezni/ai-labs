use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Email value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, ValidationError> {
        let email = email.into();
        Self::validate(&email)?;
        Ok(Email(email.to_lowercase()))
    }

    fn validate(email: &str) -> Result<(), ValidationError> {
        let email_regex = Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).unwrap();

        if !email_regex.is_match(email) {
            return Err(ValidationError::InvalidEmail);
        }

        if email.len() > 255 {
            return Err(ValidationError::EmailTooLong);
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
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

/// Username value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: impl Into<String>) -> Result<Self, ValidationError> {
        let username = username.into();
        Self::validate(&username)?;
        Ok(Username(username))
    }

    fn validate(username: &str) -> Result<(), ValidationError> {
        if username.len() < 3 {
            return Err(ValidationError::UsernameTooShort);
        }

        if username.len() > 100 {
            return Err(ValidationError::UsernameTooLong);
        }

        // Allow alphanumeric, underscore, hyphen, and dot
        let username_regex = Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap();
        if !username_regex.is_match(username) {
            return Err(ValidationError::InvalidUsername);
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.0
    }
}

/// Phone number value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: impl Into<String>) -> Result<Self, ValidationError> {
        let phone = phone.into();
        Self::validate(&phone)?;
        Ok(PhoneNumber(phone))
    }

    fn validate(phone: &str) -> Result<(), ValidationError> {
        // Basic phone validation - starts with + and contains digits
        let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
        
        if !phone_regex.is_match(phone) {
            return Err(ValidationError::InvalidPhoneNumber);
        }

        if phone.len() > 20 {
            return Err(ValidationError::PhoneNumberTooLong);
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PhoneNumber> for String {
    fn from(phone: PhoneNumber) -> Self {
        phone.0
    }
}

/// Password value object with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn new(password: impl Into<String>) -> Result<Self, ValidationError> {
        let password = password.into();
        Self::validate(&password)?;
        Ok(Password(password))
    }

    fn validate(password: &str) -> Result<(), ValidationError> {
        if password.len() < 8 {
            return Err(ValidationError::PasswordTooShort);
        }

        if password.len() > 128 {
            return Err(ValidationError::PasswordTooLong);
        }

        // At least one uppercase, one lowercase, one digit
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());

        if !has_uppercase || !has_lowercase || !has_digit {
            return Err(ValidationError::PasswordTooWeak);
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "********")
    }
}

impl From<Password> for String {
    fn from(password: Password) -> Self {
        password.0
    }
}

/// Verification token value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationToken(String);

impl VerificationToken {
    pub fn new(token: impl Into<String>) -> Self {
        VerificationToken(token.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<VerificationToken> for String {
    fn from(token: VerificationToken) -> Self {
        token.0
    }
}

/// Verification code value object (6-digit numeric)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationCode(String);

impl VerificationCode {
    pub fn new(code: impl Into<String>) -> Result<Self, ValidationError> {
        let code = code.into();
        Self::validate(&code)?;
        Ok(VerificationCode(code))
    }

    fn validate(code: &str) -> Result<(), ValidationError> {
        let code_regex = Regex::new(r"^\d{6}$").unwrap();
        
        if !code_regex.is_match(code) {
            return Err(ValidationError::InvalidVerificationCode);
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VerificationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<VerificationCode> for String {
    fn from(code: VerificationCode) -> Self {
        code.0
    }
}

/// Network ID value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkId(String);

impl NetworkId {
    pub fn new(id: impl Into<String>) -> Self {
        NetworkId(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NetworkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for NetworkId {
    fn default() -> Self {
        NetworkId(String::new())
    }
}

/// Station ID value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StationId(String);

impl StationId {
    pub fn new(id: impl Into<String>) -> Self {
        StationId(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for StationId {
    fn default() -> Self {
        StationId(String::new())
    }
}

/// Validation errors for value objects
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Email is too long (max 255 characters)")]
    EmailTooLong,
    
    #[error("Username must be at least 3 characters")]
    UsernameTooShort,
    #[error("Username is too long (max 100 characters)")]
    UsernameTooLong,
    #[error("Username can only contain alphanumeric characters, dots, underscores, and hyphens")]
    InvalidUsername,
    
    #[error("Invalid phone number format")]
    InvalidPhoneNumber,
    #[error("Phone number is too long (max 20 characters)")]
    PhoneNumberTooLong,
    
    #[error("Password must be at least 8 characters")]
    PasswordTooShort,
    #[error("Password is too long (max 128 characters)")]
    PasswordTooLong,
    #[error("Password must contain at least one uppercase, one lowercase, and one digit")]
    PasswordTooWeak,
    
    #[error("Verification code must be 6 digits")]
    InvalidVerificationCode,
}