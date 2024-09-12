use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub product_id: Option<Uuid>,
    pub product_name: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub sku: Option<String>,
}