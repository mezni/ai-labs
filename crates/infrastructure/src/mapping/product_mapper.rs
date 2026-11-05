use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::persistence::models::product_model::ProductModel;
use crate::mapping::error::MappingError;

use bio_com_domain::catalog::product::{
    Product,
    ProductId,
    Sku,
    ProductName,
};
use bio_com_domain::shared::money::Money;

pub struct ProductMapper;

impl ProductMapper {
    pub fn to_domain(
        model: ProductModel,
    ) -> Result<Product, MappingError> {
        let id = ProductId::try_from(model.id)
            .map_err(|_| MappingError::InvalidIdentifier)?;

        let sku = Sku::new(model.sku)
            .map_err(|_| MappingError::InvalidData)?;

        let name = ProductName::new(model.name)
            .map_err(|_| MappingError::InvalidData)?;

        let money = Self::map_money(
            model.price,
            model.currency,
        )?;

        Product::reconstitute(
            id,
            sku,
            name,
            model.description,
            money,
            model.is_active,
        )
        .map_err(|_| MappingError::InvalidData)
    }

    fn map_money(
        amount: BigDecimal,
        currency: String,
    ) -> Result<Money, MappingError> {
        let minor_units = amount
            .with_scale(2)
            .to_i64()
            .ok_or(MappingError::InvalidMoney)?;

        Money::new(minor_units, currency)
            .map_err(|_| MappingError::InvalidMoney)
    }
}