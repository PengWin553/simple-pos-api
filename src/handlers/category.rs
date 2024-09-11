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
    
    let result = sqlx::query_as!(
        Category,
        "SELECT * FROM categories"
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
        json!({"success": true, "data": result}).to_string()
    ))
}

pub async fn create_category(
    State(app_state): State<Arc<AppState>>,
    Json(mut category): Json<Category> 
) -> Result<(StatusCode, String), (StatusCode, String)> {

    category.category_id = Some(Uuid::new_v4());

    let data = sqlx::query_as!(
        Category,
        "INSERT INTO categories (category_id, category_name)
         VALUES ($1, $2)
         RETURNING *",
        category.category_id,
        category.category_name
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
    Json(category_update): Json<Category>
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let mut query = "UPDATE categories SET category_id = $1".to_owned();

    let mut i = 2;

    if category_update.category_name.is_some() {
        query.push_str(&format!(", category_name = ${i}"));
        i += 1
    }

    query.push_str(&format!(" WHERE category_id = $1 "));

    let mut s = sqlx::query(&query).bind(category_id);

    if category_update.category_name.is_some() {
        s = s.bind(category_update.category_name);
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

pub async fn delete_category(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<Uuid>
) -> Result<(StatusCode, String), (StatusCode, String)> {

    sqlx::query!("DELETE FROM categories WHERE category_id = $1", category_id)
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