use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde_json::json;
use uuid::Uuid;

use crate::{models::categories_model::
    Category,
    AppState
};

pub async fn get_all_category(
    State(app_state): State<Arc<AppState>>,
) -> Result<(StatusCode, String), (StatusCode, String)>{
    
    let data = sqlx::query_as!(
        Category,
        r#"
            SELECT category_id, category_name
            FROM categories
        "#
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
        json!({"success": true, "data": data}).to_string()
    ))
}

pub async fn create_category(
    State(app_state): State<Arc<AppState>>,
    Json(category): Json<Category> 
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let data = sqlx::query_as!(
        Category,
        r#"
            INSERT INTO categories (category_id, category_name)
            VALUES ($1, $2)
            RETURNING *
        "#,
        Uuid::new_v4(),
        category.category_name,
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
    ))?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": data}).to_string(),
    ))
}

pub async fn update_category(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<Uuid>,
    Json(update_category): Json<Category>
) -> Result<(StatusCode, String), (StatusCode, String)> {

    sqlx::query!(
        r#"
            UPDATE categories
            SET category_name = $1
            WHERE category_id = $2
        "#,
        update_category.category_name,
        category_id,
    )
        .execute(&app_state.db)
        .await
        .map_err(|e| {
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

pub async fn delete_category(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<Uuid>
) -> Result<(StatusCode, String), (StatusCode, String)> {

    sqlx::query!(
        r#"
            DELETE FROM categories
            WHERE category_id = $1
        "#,
        category_id
    )
        .execute(&app_state.db)
        .await
        .map_err(|e| {
           (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
           )
        })?;
    
    Ok((
        StatusCode::OK, json!({"success": true}).to_string()
    ))
}