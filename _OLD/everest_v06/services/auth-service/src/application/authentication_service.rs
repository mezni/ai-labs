use crate::AppState;
use std::sync::Arc;

pub struct AuthenticationService {
    state: Arc<AppState>,
}

impl AuthenticationService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn login(&self) -> String {
        format!(
            "login placeholder for realm: {}",
            self.state.config.keycloak_realm
        )
    }

    pub async fn logout(&self) -> String {
        "logout placeholder".to_string()
    }

    pub async fn refresh(&self) -> String {
        "refresh placeholder".to_string()
    }

    pub async fn validate(&self) -> String {
        "validate placeholder".to_string()
    }
}
