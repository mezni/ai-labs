use bio_com_application::ports::integration::payment::PaymentGateway;
use bio_com_domain::payments::PaymentRequest;
use bio_com_domain::payments::PaymentResult;

pub struct PaymentGatewayAdapter;

#[async_trait]
impl PaymentGateway for PaymentGatewayAdapter {
    async fn authorize(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, bio_com_application::ApplicationError> {
        // TODO: Implement payment provider integration
        Ok(PaymentResult {
            success: true,
            transaction_id: uuid::Uuid::new_v4().to_string(),
            amount: request.amount,
            currency: request.currency,
        })
    }
}