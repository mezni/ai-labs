use crate::AppState;
use crate::application::dtos::registration::{
    RegisterRequest, RegisterResponse, ResendVerificationRequest, ResendVerificationResponse,
    VerifyRequest, VerifyResponse,
};
use crate::core::errors::AppError;
use std::sync::Arc;

pub struct RegistrationService {
    state: Arc<AppState>,
}

impl RegistrationService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn register_user(&self, req: RegisterRequest) -> Result<RegisterResponse, AppError> {
        todo!("Implement register_user")
    }

    pub async fn verify_registration(
        &self,
        req: VerifyRequest,
    ) -> Result<VerifyResponse, AppError> {
        todo!("Implement verify_registration")
    }

    pub async fn resend_verification(
        &self,
        req: ResendVerificationRequest,
    ) -> Result<ResendVerificationResponse, AppError> {
        todo!("Implement resend_verification")
    }
}
