use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct TransactionModel {
    pub transaction_id: Option<String>,
    pub transaction_date: Option<DateTime<Utc>>,
    pub total_price: Option<Decimal>,
    pub item_count: Option<i32>,
    pub transaction_items: Value,
}

#[derive(Debug, Deserialize)]
pub struct TransactionInputModel {
    pub transaction_items: Vec<TransactionItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionItem {
    pub product_name: String,
    pub product_category: String,
    pub quantity: u32,
    pub price: f64,
}