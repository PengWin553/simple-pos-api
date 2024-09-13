use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::{get, patch}, Router};

use crate::{
    handlers::{
        category::{create_category, delete_category, get_all_categories, update_category},
        product::{create_product, delete_product, get_all_products, get_product, update_product},
        transaction::{create_transaction, get_all_transactions}},
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
        .route("/", get(get_all_products).post(create_product))
        .route("/:product_id", get( get_product)
            .patch(update_product)
            .delete(delete_product))
        .with_state(app_state)
}

pub fn category_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_all_categories).post(create_category))
        .route("/:category_id", patch(update_category).delete(delete_category))
        .with_state(app_state)
}

pub fn transaction_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_all_transactions).post(create_transaction))
        .with_state(app_state)
}