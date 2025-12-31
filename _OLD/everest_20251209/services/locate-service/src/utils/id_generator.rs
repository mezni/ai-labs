use nanoid::nanoid;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn generate_id() -> String {
    nanoid!(32, &ALPHABET)
}

pub fn generate_review_id() -> String {
    format!("REV-{}", nanoid!(20, &ALPHABET))
}

pub fn generate_station_id() -> String {
    format!("STA-{}", nanoid!(20, &ALPHABET))
}
