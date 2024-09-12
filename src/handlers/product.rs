use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use uuid::Uuid;
use std::sync::Arc;

use crate::{models::products_model::
    Product, AppState
};

//TODO: offset and limit
pub async fn get_all_product(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let result = sqlx::query_as!(
        Product,
        r#"
            SELECT * FROM products
            ORDER BY product_id
            
        "#,
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success" : false, "message" : e.to_string()}),)
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(json!({"success" : true, "data" : result})),
    ))
}

pub async fn create_product(
    State(app_state): State<Arc<AppState>>,
    Json(product): Json<Product>,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let result = sqlx::query_as!(
        Product,
        r#"
            INSERT INTO products (product_id, product_name, price, stock, sku)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
        Uuid::new_v4(),
        product.product_name,
        product.price,
        product.stock,
        product.sku,
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
) ->  Result<(StatusCode, String), (StatusCode, String)> {
    
    let mut query = "UPDATE products SET product_id = $1".to_owned();

    let mut  i = 2;

    if update_product.product_name.is_some() {
        query.push_str(&format!(", product_name = ${i}"));
        i += 1
    };

    if update_product.price.is_some() {
        query.push_str(&format!(", price = ${i}"));
        i += 1
    };

    if update_product.stock.is_some() {
        query.push_str(&format!(", stock = ${i}"));
        i += 1
    };

    if update_product.sku.is_some() {
        query.push_str(&format!(", sku = ${i}"));
    };

    query.push_str(&format!(" WHERE product_id = $1 "));

    let mut s = sqlx::query(&query).bind(product_id);

    if update_product.product_name.is_some() {
        s = s.bind(update_product.product_name);
    }

    if update_product.price.is_some() {
        s = s.bind(update_product.price);
    }

    if update_product.stock.is_some() {
        s = s.bind(update_product.stock);
    }

    if update_product.sku.is_some() {
        s = s.bind(update_product.sku);
    }

    s.execute(&app_state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({"success": true}).to_string(),
    ))

}

pub async fn delete_product(
    State(app_state): State<Arc<AppState>>,
    Path(product_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    sqlx::query!(
        r#"
            DELETE FROM products
            WHERE product_id = $1
        "#,
        product_id,
    )
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
