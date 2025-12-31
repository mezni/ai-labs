use nanoid::nanoid;

const ALPHABET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z',
];

pub fn generate_user_id() -> String {
    format!("USR-{}", nanoid!(16, &ALPHABET))
}

pub fn generate_audit_id() -> String {
    format!("AUD-{}", nanoid!(16, &ALPHABET))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_format() {
        let id = generate_user_id();
        assert!(id.starts_with("USR-"));
        assert_eq!(id.len(), 20);
    }

    #[test]
    fn test_audit_id_format() {
        let id = generate_audit_id();
        assert!(id.starts_with("AUD-"));
        assert_eq!(id.len(), 20);
    }
}