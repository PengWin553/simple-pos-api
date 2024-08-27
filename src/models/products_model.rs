use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetProductModel {
    pub product_id: Option<String>,
    pub product_name: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub sku: Option<String>,
    pub category_name: Option<String>,
    pub product_image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostProductModel {
    pub product_id: Option<String>,
    pub product_name: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub sku: Option<String>,
    pub category_id: Option<String>,
    pub product_image: Option<String>,
}