use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Refund {
    pub order_id: String,
    pub amount: f64,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundResult {
    pub status: String,
    pub order_id: String,
    pub amount: f64,
    pub reason: String,
    pub message: String,
}
