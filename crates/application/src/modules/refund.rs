#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refund;

impl Default for Refund {
    fn default() -> Self {
        Self::new()
    }
}

impl Refund {
    pub fn new() -> Self {
        Self
    }
}