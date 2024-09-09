use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use product::product_route;

mod product;

pub fn app_router() -> Router {
    Router::new()
    .route("/",get(root))
    .merge(product_route())
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