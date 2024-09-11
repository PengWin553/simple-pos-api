use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde_json::json;
use uuid::Uuid;
use std::sync::Arc;

use crate::{models::products_model::
    Product, AppState
};

pub async fn get_all_product(
    State(app_state): State<Arc<AppState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let result = sqlx::query_as!(
        Product,
        "SELECT * FROM products"
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": result}).to_string(),
    ))
}

pub async fn create_product(
    State(app_state): State<Arc<AppState>>,
    Json(product): Json<Product>,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let result = sqlx::query_as!(
        Product,
        "
        INSERT INTO products (product_id, product_name, price, stock, sku)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        ",
        product.product_id,
        product.product_name,
        product.price,
        product.stock,
        product.sku
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": result}).to_string(),
    ))
}

pub async fn update_product(
    State(app_state): State<Arc<AppState>>,
    Path(product_id): Path<Uuid>,
    Json(update_product): Json<Product>,
) {
    todo!()
}

pub async fn delete_product(
    State(app_state): State<Arc<AppState>>,
    Path(product_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    
    sqlx::query!("DELETE FROM products WHERE product_id = $1", product_id,)
        .execute(&app_state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((StatusCode::OK, json!({"success":true}).to_string()))
}
