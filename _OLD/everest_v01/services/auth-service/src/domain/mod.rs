// src/domain/mod.rs
pub mod events;
pub mod registration;
pub mod repositories;
pub mod token;
pub mod user;
pub mod value_objects;

// Re-export commonly used items
pub use events::*;
pub use registration::{RegistrationStatus, UserRegistration};
pub use repositories::{
    AuditLogFilters, AuditLogRepository, LoginAuditLog, RegistrationRepository, SortOrder,
    UserFilters, UserRepository,
};
pub use token::{
    LoginCredentials, PasswordChangeRequest, PasswordResetRequest, PasswordResetToken, Session,
    TokenResponse, TokenType,
};
pub use user::{User, UserBuilder};
pub use value_objects::{
    Email, NetworkId, PersonName, PhoneNumber, StationId, UserRole, UserSource, Username,
};
