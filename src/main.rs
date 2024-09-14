use std::{error::Error, sync::Arc};
use config::init_config;
use routes::app_router;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod config;
mod routes;
mod handlers;
mod models;
mod middlewares;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub env: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("Unable to access .env file");

    let config = init_config().await;

    let listener = TcpListener::bind(config.address)
    .await
    .expect("Could not create tcp listener");

    println!("Listening on http://{}", listener.local_addr().unwrap());

    let app_state = Arc::new(AppState {
        db: config.pool.clone(),
        env: config.jwt.clone(),
    });

    let app = app_router(app_state);

    axum::serve(listener, app)
    .await
    .expect("Error serving application");

    Ok(())
}
