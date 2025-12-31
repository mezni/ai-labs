use crate::core::constants::INVITATION_CODE_LENGTH;
use nanoid::nanoid;

pub fn generate_invitation_code() -> String {
    nanoid!(INVITATION_CODE_LENGTH)
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 3
}

pub fn validate_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 50 && username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
}