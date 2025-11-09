use uuid::Uuid;

pub fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}
