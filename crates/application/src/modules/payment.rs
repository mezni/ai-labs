#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment;

impl Default for Payment {
    fn default() -> Self {
        Self::new()
    }
}

impl Payment {
    pub fn new() -> Self {
        Self
    }
}