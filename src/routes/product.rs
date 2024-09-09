use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

pub fn product_route() -> Router {
    Router::new()
    .route("/product", get(get_all_product))
}

#[derive(Serialize)]
struct Product{
    name: &'static str,
    stock: i16,
}

async fn get_all_product() -> (StatusCode, Json<Product>) {
    let product = Product {
        name: "test",
        stock: 0
    };
    
    (StatusCode::OK, Json(product))
}