use crate::AppState;
use std::sync::Arc;

#[derive(Clone)]
pub struct AdminService {
    state: Arc<AppState>,
}

impl AdminService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn list_users(&self) -> String {
        "list_users placeholder".to_string()
    }

    pub async fn get_user(&self, id: String) -> String {
        format!("get_user placeholder for id: {}", id)
    }

    pub async fn create_user(&self) -> String {
        "create_user placeholder".to_string()
    }

    pub async fn update_user(&self, id: String) -> String {
        format!("update_user placeholder for id: {}", id)
    }

    pub async fn delete_user(&self, id: String) -> String {
        format!("delete_user placeholder for id: {}", id)
    }
}
