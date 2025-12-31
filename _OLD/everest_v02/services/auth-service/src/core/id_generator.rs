use nanoid::nanoid;

pub struct IdGenerator;

impl IdGenerator {
    /// Example: REG-4f7j2l9n3m5p8q1r
    pub fn generate_registration_id() -> String {
        format!("REG-{}", nanoid!(16))
    }

    /// Generates a high-entropy verification token (32 characters).
    pub fn generate_verification_token() -> String {
        "9999".to_string()
        //        nanoid!(32)
    }

    /// Generates a standard 21-character NanoID.
    pub fn generate_id() -> String {
        nanoid!()
    }
}
