// src/application/auth_queries.rs
use crate::core::errors::AppResult;
use crate::domain::repositories::UserRepository;
use crate::infrastructure::TokenBlacklist;
use std::sync::Arc;

pub struct AuthQueries {
    user_repo: Arc<dyn UserRepository>,
    token_blacklist: Arc<TokenBlacklist>,
}

impl AuthQueries {
    pub fn new(user_repo: Arc<dyn UserRepository>, token_blacklist: Arc<TokenBlacklist>) -> Self {
        Self {
            user_repo,
            token_blacklist,
        }
    }

    /// Check if token is blacklisted
    pub fn is_token_blacklisted(&self, token: &str) -> bool {
        self.token_blacklist.is_blacklisted(token)
    }

    /// Validate user exists and is active
    pub async fn validate_user_active(&self, user_id: &str) -> AppResult<bool> {
        if let Some(user) = self.user_repo.find_by_id(user_id).await? {
            Ok(user.is_active && !user.is_deleted())
        } else {
            Ok(false)
        }
    }
}
