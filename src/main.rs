use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::info;

mod common;
mod handler;
mod middleware;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let listener: TcpListener;
    dotenv::dotenv().ok();

    match TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener_ok) => { listener = listener_ok; }
        Err(e) => {
            info!("Error binding to port: {}", e);
            std::process::exit(1);
        }
    }

    info!("Listening on {}", listener.local_addr().unwrap());

    let app = routes::app().await;
    match axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            info!("Error starting server: {}", e);
            std::process::exit(1);
        }
    }
}
