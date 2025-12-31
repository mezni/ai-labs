use crate::AppState;
use crate::application::dtos::registration::{
    RegisterUserRequest, ResendVerificationRequest, VerifyUserRequest,
};
use std::sync::Arc;

pub struct RegistrationService {
    state: Arc<AppState>,
}

impl RegistrationService {
    // This is the "associated item" the compiler says is missing
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn register_user(&self, req: RegisterUserRequest) -> String {
        format!("Registration successful for {}", req.username)
    }

    pub async fn verify_registration(&self, req: VerifyUserRequest) -> String {
        format!("Verification successful for {}", req.email)
    }

    pub async fn resend_verification(&self, req: ResendVerificationRequest) -> String {
        format!("Resent token to {}", req.email)
    }
}
