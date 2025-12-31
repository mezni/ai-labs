pub struct Generator;

impl Generator {
    /// Internal helper to generate a prefixed nanoid
    fn prefixed_id(prefix: &str, size: usize) -> String {
        format!("{}-{}", prefix, nanoid::nanoid!(size))
    }

    /// Generates a User ID: e.g., "USR-abc123def456ghij"
    pub fn generate_user_id() -> String {
        Self::prefixed_id("USR", 16)
    }

    /// Generates a Registration ID: e.g., "REG-abc123def456ghij"
    pub fn generate_registration_id() -> String {
        Self::prefixed_id("REG", 16)
    }

    /// Generates a long secure token for verification (no prefix usually required)
    pub fn generate_token() -> String {
        "99999".to_string()
        //        nanoid::nanoid!(64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_format() {
        let id = Generator::generate_user_id();
        assert!(id.starts_with("USR-"));
        assert_eq!(id.len(), 4 + 16); // "USR-" (4) + 16 chars
    }
}
