use crate::models::{Refund, RefundResult};

pub fn process_refund(refund: &Refund) -> RefundResult {
    RefundResult {
        status: "approved".to_string(),
        order_id: refund.order_id.clone(),
        amount: refund.amount,
        reason: refund.reason.clone(),
        message: "Refund processed successfully".to_string(),
    }
}
