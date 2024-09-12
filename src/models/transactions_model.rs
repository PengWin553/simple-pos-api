use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct TransactionModel {
    pub transaction_id: Option<Uuid>,
    pub transaction_date: Option<DateTime<Utc>>,
    pub total_price: Option<Decimal>,
    pub transaction_items: Value,
}

#[derive(Debug, Deserialize)]
pub struct TransactionInputModel {
    pub total_price: Option<Decimal>,
    pub transaction_items: Vec<TransactionItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionItem {
    pub quantity: u32,
    pub price: f64,
}