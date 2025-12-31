use crate::AppState;
use std::sync::Arc;

#[derive(Clone)]
pub struct InvitationService {
    state: Arc<AppState>,
}

impl InvitationService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn create(&self) -> String {
        "create_invitation placeholder".into()
    }
    pub async fn list(&self) -> String {
        "list_invitations placeholder".into()
    }
    pub async fn get(&self, code: String) -> String {
        format!("get_invitation: {}", code)
    }
    pub async fn accept(&self, code: String) -> String {
        format!("accept_invitation: {}", code)
    }
    pub async fn cancel(&self, code: String) -> String {
        format!("cancel_invitation: {}", code)
    }
}
