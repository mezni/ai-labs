#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: String,
    pub customer_id: String,
    pub total_amount_cents: u64,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefundRequest {
    pub id: String,
    pub customer_id: String,
    pub order_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefundDecision {
    Eligible { amount_cents: u64, reason: String },
    Ineligible { reason: String },
    RequiresHumanReview { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefundStatus {
    Requested,
    UnderReview,
    Approved,
    Rejected,
    Processed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_stores_amount_in_cents() {
        let order = Order {
            id: "ORD-1001".to_string(),
            customer_id: "CUST-001".to_string(),
            total_amount_cents: 4_999,
            status: OrderStatus::Delivered,
        };

        assert_eq!(order.total_amount_cents, 4_999);
    }

    #[test]
    fn refund_request_references_order_and_customer() {
        let request = RefundRequest {
            id: "REF-001".to_string(),
            customer_id: "CUST-001".to_string(),
            order_id: "ORD-1001".to_string(),
            reason: "Damaged item".to_string(),
        };

        assert_eq!(request.customer_id, "CUST-001");
        assert_eq!(request.order_id, "ORD-1001");
    }

    #[test]
    fn refund_decision_can_require_human_review() {
        let decision = RefundDecision::RequiresHumanReview {
            reason: "Evidence required".to_string(),
        };

        assert!(matches!(
            decision,
            RefundDecision::RequiresHumanReview { .. }
        ));
    }

    #[test]
    fn refund_status_can_be_under_review() {
        let status = RefundStatus::UnderReview;

        assert_eq!(status, RefundStatus::UnderReview);
    }
}
