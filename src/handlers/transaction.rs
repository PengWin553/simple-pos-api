use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{models::transactions_model::{TransactionInputModel, TransactionModel}, AppState};


pub async fn get_all_transaction(
    State(app_state): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    
    let data = sqlx::query_as!(
        TransactionModel,
        r#"
            SELECT * FROM transactions
        "#
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"succes": false, "message": e.to_string()}))
            )
        })?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"succes": true, "data": data})),
    ))
}

pub async fn create_transaction(
    State(app_state): State<Arc<AppState>>,
    Json(transactions): Json<TransactionInputModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let total_price = transactions.total_price;
    let transaction_items = json!(transactions.transaction_items);

    let result = sqlx::query_as!(
        TransactionModel,
        r#"
            INSERT INTO transactions (transaction_id, transaction_date, total_price, transaction_items)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
        Uuid::new_v4(),
        Utc::now(),
        total_price,
        transaction_items,
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(|e|{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": e.to_string()}))
            )
        })?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"success": true, "data": result})),
    ))
}