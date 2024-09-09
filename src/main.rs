use routes::app_router;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap();

    let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not create tcp listener");

    println!("Listening on http://{}", listener.local_addr().unwrap());

    let app = app_router();

    axum::serve(listener, app)
    .await
    .expect("Error serving application");
}
