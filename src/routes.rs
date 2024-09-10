use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::{delete, get}, Router};

use crate::{
    handlers::product::{create_product, delete_product, get_all_product, update_product},
    AppState
};

pub fn app_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/",get(root))
        .merge(product_route(app_state))
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
        .route("/product", get(get_all_product).post(create_product))
        .route("/product/:product_id", delete(delete_product).patch(update_product))
        .with_state(app_state)
}