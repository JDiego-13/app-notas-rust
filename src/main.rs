use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Este Server viene de `hyper::server::Server`, pero lo usa Axum indirectamente
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
