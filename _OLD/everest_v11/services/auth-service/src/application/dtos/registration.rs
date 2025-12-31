use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct VerifyRequest {
    pub email: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResendVerificationRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}