use super::entities::{User, UserRegistration};
use crate::core::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait RegistrationService: Send + Sync {
    async fn register(
        &self,
        email: String,
        username: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        source: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<UserRegistration>;

    async fn verify(&self, token: String) -> AppResult<User>;

    async fn resend_verification(&self, email: String) -> AppResult<()>;
}

#[async_trait]
pub trait AuthenticationService: Send + Sync {
    async fn login(&self, username: String, password: String) -> AppResult<LoginResponse>;
    async fn logout(&self, refresh_token: String) -> AppResult<()>;
    async fn refresh_token(&self, refresh_token: String) -> AppResult<LoginResponse>;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UserInfo {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub role: String,
}
