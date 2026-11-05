#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer;

impl Default for Customer {
    fn default() -> Self {
        Self::new()
    }
}

impl Customer {
    pub fn new() -> Self {
        Self
    }
}
