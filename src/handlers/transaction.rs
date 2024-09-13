use std::sync::Arc;
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{models::{filter_model::FilterOptionsModel, transactions_model::{TransactionInputModel, TransactionModel}}, AppState};


pub async fn get_all_transactions(
    State(app_state): State<Arc<AppState>>,
    filter_options: Option<Query<FilterOptionsModel>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let Query(opts) = filter_options.unwrap_or_default();

    let limit = opts.limit.unwrap_or(20);
    let offset = (opts.offset.unwrap_or(1) - 1) * limit;

    let total_transactions: Option<i64> = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*)
            FROM products
        "#
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success" : false,
                "message" : e.to_string(),
            })),
        )
    })?;
    
    let transactions = sqlx::query_as!(
        TransactionModel,
        r#"
            SELECT * FROM transactions
            OFFSET $1
            LIMIT $2
        "#,
        offset,
        limit,
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "succes": false,
                    "message": e.to_string(),
                })),
            )
        })?;
    
    let json_response = json!({
        "succes": true,
        "data": transactions,
        "total": total_transactions,
        "offset": offset,
        "limit": limit,
    });
    
    Ok((
        StatusCode::OK,
        Json(json_response),
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
                Json(json!({
                    "success": false,
                    "message": e.to_string(),
                })),
            )
        })?;
    
    Ok((
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": result,
        })),
    ))
}