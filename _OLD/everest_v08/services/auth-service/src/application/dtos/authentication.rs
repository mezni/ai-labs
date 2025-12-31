use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl AuthResponse {
    pub fn placeholder() -> Self {
        Self {
            access_token: "access".into(),
            refresh_token: "refresh".into(),
        }
    }
}
