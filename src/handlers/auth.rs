use std::sync::Arc;
use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::Utc;

use crate::{models::auth_model::{LoginModel, SignupModel, TokenClaims}, AppState};

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(credentials): Json<LoginModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = sqlx::query!(
        r#"
            SELECT id, username, password
            FROM accounts
            WHERE username = $1
        "#,
        credentials.username.to_ascii_lowercase(),
    )
    .fetch_optional(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": format!("Database error: {}", e),
            })),
        )
    })?
    .ok_or_else(|| {
        let error_response = json!({
            "success": false,
            "message": "Invalid email or password",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(credentials.password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = json!({
            "success": false,
            "message": "Invalid email or password"
        });
        return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&app_state.env.as_bytes()),
    ).map_err(|_| {
        let error_response = json!({
            "success": false,
            "message": "Failed to generate token"
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let response_body = json!({
        "success": true,
        "token": token
    });

    Ok((StatusCode::OK, Json(response_body)))
}

pub async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(credentials): Json<SignupModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = sqlx::query!(
        r#"
            SELECT username
            FROM accounts
            WHERE username = $1
        "#,
        credentials.username.to_ascii_lowercase(),
    )
    .fetch_optional(&app_state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": format!("Database error: {}", e),
            })),
        )
    })?;

    if user.is_some() {
        let error_response = json!({
            "success": false,
            "message": "Username already taken",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(credentials.password.as_bytes(), &salt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": format!("Password hashing error: {}", e),
                })),
            )
        })?;

    sqlx::query_as!(
        SignupModel,
        r#"
            INSERT INTO accounts (id, full_name, username, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        credentials.full_name,
        credentials.username.to_ascii_lowercase(),
        hashed_password.to_string(),
        Utc::now(),
        Utc::now(),
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