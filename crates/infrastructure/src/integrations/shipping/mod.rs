use bio_com_application::ports::integration::shipping::ShippingAdapter as ShippingPort;
use bio_com_domain::shipping::ShipmentRequest;

pub struct ShippingAdapter;

#[async_trait]
impl ShippingPort for ShippingAdapter {
    async fn create_shipment(
        &self,
        request: ShipppingRequest,
    ) -> Result<bio_com_domain::shipping::ShipmentResponse, bio_com_application::ApplicationError> {
        // TODO: Implement shipping provider integration
        Ok(bio_com_domain::shipping::ShipmentResponse {
            shipment_id: uuid::Uuid::new_v4().to_string(),
            tracking_number: uuid::Uuid::new_v4().to_string(),
            status: "created".into(),
        })
    }
}