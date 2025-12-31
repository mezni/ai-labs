use crate::core::constants::NANOID_LENGTH;
use nanoid::nanoid;

pub fn generate_id(prefix: &str) -> String {
    format!("{}-{}", prefix, nanoid!(NANOID_LENGTH))
}

pub fn generate_verification_token() -> String {
    "99999".to_string()
    //    nanoid!(64)
}
