use std::sync::Arc;
use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{models::{categories_model::
    CategoryModel, filter_model::FilterOptionsModel},
    AppState
};

pub async fn get_all_categories(
    State(app_state): State<Arc<AppState>>,
    filter_options: Option<Query<FilterOptionsModel>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)>{

    let Query(opts) = filter_options.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.offset.unwrap_or(1) - 1) * limit;

    let total_categories: Option<i64> = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*)
            FROM categories
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
    
    let categories = sqlx::query_as!(
        CategoryModel,
        r#"
            SELECT category_id, category_name
            FROM categories
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
                    "success": false,
                    "message": e.to_string(),
                })),
            )
        })?;
    
    let json_response = json!({
        "success": true,
        "data": categories,
        "total": total_categories,
        "offset": offset,
        "limit": limit,
    });

    Ok((
        StatusCode::OK,
        Json(json_response),
    ))
}

pub async fn create_category(
    State(app_state): State<Arc<AppState>>,
    Json(category): Json<CategoryModel> 
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let category = sqlx::query_as!(
        CategoryModel,
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
        Json(json!({
            "success": false,
            "message": e.to_string(),
        })),
    ))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "data": category,
        })),
    ))
}

pub async fn update_category(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<Uuid>,
    Json(update_category): Json<CategoryModel>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

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
        }))
    ))
}

pub async fn delete_category(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<Uuid>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

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
        })),
    ))
}