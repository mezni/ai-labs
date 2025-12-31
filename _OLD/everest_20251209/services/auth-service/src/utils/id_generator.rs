use nanoid::nanoid;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn generate_user_id() -> String {
    let id = nanoid!(21, &ALPHABET);
    format!("USR{}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_user_id() {
        let id = generate_user_id();
        assert!(id.starts_with("USR"));
        assert_eq!(id.len(), 24); // USR (3) + 21 characters
    }

    #[test]
    fn test_unique_ids() {
        let id1 = generate_user_id();
        let id2 = generate_user_id();
        assert_ne!(id1, id2);
    }
}
