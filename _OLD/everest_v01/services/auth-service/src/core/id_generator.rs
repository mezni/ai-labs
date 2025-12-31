// src/core/id_generator.rs
use nanoid::nanoid;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const ID_SIZE: usize = 18; // Length of the random part

pub struct IdGenerator;

impl IdGenerator {
    /// Generate a new ID with the given prefix
    /// Format: PREFIX-XXXXXXXXXXXXXXXXXX
    /// Example: USR-ABC123XYZ456DEF789
    pub fn generate(prefix: &str) -> String {
        let random_part = nanoid!(ID_SIZE, &ALPHABET);
        format!("{}-{}", prefix, random_part)
    }

    /// Generate a user ID
    pub fn user_id() -> String {
        Self::generate(crate::core::constants::PREFIX_USER)
    }

    /// Generate a network ID
    pub fn network_id() -> String {
        Self::generate(crate::core::constants::PREFIX_NETWORK)
    }

    /// Generate a station ID
    pub fn station_id() -> String {
        Self::generate(crate::core::constants::PREFIX_STATION)
    }

    /// Generate a registration ID
    pub fn registration_id() -> String {
        Self::generate(crate::core::constants::PREFIX_REGISTRATION)
    }

    /// Validate ID format
    pub fn is_valid_format(id: &str, expected_prefix: &str) -> bool {
        if !id.starts_with(&format!("{}-", expected_prefix)) {
            return false;
        }

        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() != 2 {
            return false;
        }

        parts[1].len() == ID_SIZE && parts[1].chars().all(|c| ALPHABET.contains(&c))
    }

    /// Extract prefix from ID
    pub fn extract_prefix(id: &str) -> Option<&str> {
        id.split('-').next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_user_id() {
        let id = IdGenerator::user_id();
        assert!(id.starts_with("USR-"));
        assert_eq!(id.len(), 22); // USR- + 18 chars
    }

    #[test]
    fn test_validate_format() {
        let id = IdGenerator::user_id();
        assert!(IdGenerator::is_valid_format(&id, "USR"));
        assert!(!IdGenerator::is_valid_format(&id, "NET"));
        assert!(!IdGenerator::is_valid_format("invalid", "USR"));
    }

    #[test]
    fn test_extract_prefix() {
        let id = "USR-ABC123XYZ456DEF789";
        assert_eq!(IdGenerator::extract_prefix(id), Some("USR"));
    }
}
