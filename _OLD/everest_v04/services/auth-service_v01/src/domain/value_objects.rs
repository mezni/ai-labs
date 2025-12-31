use crate::core::constants::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH};
use crate::core::errors::AppError;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, AppError> {
        let email = email.trim().to_lowercase();
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        if !email_regex.is_match(&email) {
            return Err(AppError::ValidationError("Invalid email format".to_string()));
        }

        Ok(Email(email))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, AppError> {
        let username = username.trim().to_string();

        if username.len() < 3 {
            return Err(AppError::ValidationError(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.len() > 50 {
            return Err(AppError::ValidationError(
                "Username must be at most 50 characters".to_string(),
            ));
        }

        let username_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if !username_regex.is_match(&username) {
            return Err(AppError::ValidationError(
                "Username can only contain letters, numbers, underscores, and hyphens".to_string(),
            ));
        }

        Ok(Username(username))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self, AppError> {
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(AppError::ValidationError(format!(
                "Password must be at least {} characters",
                MIN_PASSWORD_LENGTH
            )));
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(AppError::ValidationError(format!(
                "Password must be at most {} characters",
                MAX_PASSWORD_LENGTH
            )));
        }

        // Check for at least one uppercase, one lowercase, one number
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());

        if !has_uppercase || !has_lowercase || !has_digit {
            return Err(AppError::ValidationError(
                "Password must contain at least one uppercase letter, one lowercase letter, and one number".to_string(),
            ));
        }

        Ok(Password(password))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("test@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::new("invalid-email".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_valid_username() {
        let username = Username::new("john_doe123".to_string());
        assert!(username.is_ok());
    }

    #[test]
    fn test_short_username() {
        let username = Username::new("ab".to_string());
        assert!(username.is_err());
    }

    #[test]
    fn test_valid_password() {
        let password = Password::new("SecurePass123".to_string());
        assert!(password.is_ok());
    }

    #[test]
    fn test_weak_password() {
        let password = Password::new("weakpass".to_string());
        assert!(password.is_err());
    }
}