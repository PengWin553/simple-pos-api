use axum::{extract::multipart::Field, http::StatusCode, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::AppState;

pub async fn upload_image<'a>(
    field: Field<'a>,
    app_state: &'a AppState,
) -> Result<String, (StatusCode, Json<Value>)> {
    
    let file = field.file_name().ok_or_else(|| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!("File name missing")),
    ))?.to_string();

    let content = field.bytes().await.map_err(|_| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!("Failed to read file content")),
    ))?;

    let extension = file.rsplit('.').next().unwrap_or("");
    let s3_path = format!("uploads/{}.{}", Uuid::new_v4(), extension);

    app_state.s3.put_object(s3_path.clone(), content.as_ref())
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ))?;
        
    Ok(s3_path)
}
