use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use std::{error::Error, sync::Arc};
use config::init_config;
use routes::app_router;
use s3::Bucket;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod config;
mod routes;
mod handlers;
mod models;
mod middlewares;
mod services;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub env: String,
    pub s3: Box<Bucket>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer().pretty().with_writer(std::io::stdout))
        .init();

    let config = init_config().await;

    let listener = TcpListener::bind(config.address)
    .await
    .expect("Could not create tcp listener");

    println!("Listening on http://{}", listener.local_addr().unwrap());

    let app_state = Arc::new(AppState {
        db: config.pool.clone(),
        env: config.jwt.clone(),
        s3: config.s3.clone(),
    });

    let app = app_router(app_state);

    axum::serve(listener, app)
    .with_graceful_shutdown(services::shutdown_service::shutdown_signal())
    .await
    .expect("Error serving application");

    Ok(())
}
