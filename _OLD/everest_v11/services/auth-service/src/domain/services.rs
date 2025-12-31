// Domain services for complex business logic that doesn't fit in entities

use crate::domain::entities::User;

pub struct UserDomainService;

impl UserDomainService {
    pub fn can_be_deleted(user: &User) -> bool {
        // Business rule: Deleted users can't be deleted again
        user.deleted_at.is_none()
    }

    pub fn is_active(user: &User) -> bool {
        user.status == "active" && user.deleted_at.is_none()
    }

    pub fn can_login(user: &User) -> bool {
        Self::is_active(user)
    }
}