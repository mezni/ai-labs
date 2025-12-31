use crate::domain::{registration::UserRegistration, user::User};
use uuid::Uuid;

pub trait UserRegistrationRepository {
    fn save(&self, registration: &UserRegistration);
    fn find_by_token(&self, token: &str) -> Option<UserRegistration>;
    fn exists_pending_by_email(&self, email: &str) -> bool;
}

pub trait UserRepository {
    fn save(&self, user: &User);
    fn find_by_id(&self, id: Uuid) -> Option<User>;
    fn find_by_keycloak_id(&self, keycloak_id: &str) -> Option<User>;
}
