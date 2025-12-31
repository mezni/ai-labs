use crate::domain::{
    registration::UserRegistration,
    repositories::UserRegistrationRepository,
};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

/// Placeholder in-memory implementation
/// Replace with tokio-postgres / deadpool later
pub struct PostgresUserRegistrationRepository {
    store: Mutex<HashMap<Uuid, UserRegistration>>,
}

impl PostgresUserRegistrationRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl UserRegistrationRepository for PostgresUserRegistrationRepository {
    fn save(&self, registration: &UserRegistration) {
        self.store
            .lock()
            .unwrap()
            .insert(registration.id(), registration.clone());
    }

    fn find_by_token(&self, token: &str) -> Option<UserRegistration> {
        self.store
            .lock()
            .unwrap()
            .values()
            .find(|r| r.verification_token().value() == token)
            .cloned()
    }

    fn exists_pending_by_email(&self, email: &str) -> bool {
        self.store
            .lock()
            .unwrap()
            .values()
            .any(|r| r.email().value() == email)
    }

    fn find_by_id(&self, id: Uuid) -> Option<UserRegistration> {
        self.store.lock().unwrap().get(&id).cloned()
    }
}
