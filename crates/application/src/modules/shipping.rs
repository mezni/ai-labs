#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shipping;

impl Default for Shipping {
    fn default() -> Self {
        Self::new()
    }
}

impl Shipping {
    pub fn new() -> Self {
        Self
    }
}