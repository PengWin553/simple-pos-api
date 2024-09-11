use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::{get, patch}, Router};

use crate::{
    handlers::{
        category::{create_category, delete_category, get_all_category, update_category},
        product::{create_product, delete_product, get_all_product, update_product},
        transaction::get_all_transaction},
    AppState
};

pub fn app_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/",get(root))
        .nest("/api/product", product_route(app_state.clone()))
        .nest("/api/category", category_route(app_state.clone()))
        .nest("/api/transaction", transaction_route(app_state.clone()))
        .fallback(handler_404)
}

async fn root() -> &'static str {
    "Server is running"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found"
    )
}

pub fn product_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_all_product).post(create_product))
        .route("/:product_id", patch(update_product).delete(delete_product))
        .with_state(app_state)
}

pub fn category_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_all_category).post(create_category))
        .route("/:category_id", patch(update_category).delete(delete_category))
        .with_state(app_state)
}

pub fn transaction_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_all_transaction))
        .with_state(app_state)
}