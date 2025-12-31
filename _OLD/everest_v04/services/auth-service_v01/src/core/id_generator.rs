use crate::core::constants::NANOID_LENGTH;
use nanoid::nanoid;

pub fn generate_id(prefix: &str) -> String {
    format!("{}-{}", prefix, nanoid!(NANOID_LENGTH))
}

pub fn generate_verification_token() -> String {
    nanoid!(64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id("USR");
        assert!(id.starts_with("USR-"));
        assert_eq!(id.len(), 4 + NANOID_LENGTH); // prefix + dash + nanoid
    }

    #[test]
    fn test_generate_verification_token() {
        let token = generate_verification_token();
        assert_eq!(token.len(), 64);
    }
}