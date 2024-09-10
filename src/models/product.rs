use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct Product {
    pub product_id: Uuid,
    pub product_name: String,
    pub price: Decimal,
    pub stock: i32,
    pub sku: String,
}