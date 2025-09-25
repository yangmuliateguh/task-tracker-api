use axum::{http, routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber;
use tracing::info;
use tower_http::trace::TraceLayer;
use tokio::net::TcpListener;
use uuid::Uuid;

async fn health() -> &'static str {
    info!("Health check diterima");
    "Ok"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("task_tracker_api=info")
        .compact()
        .init();

    tracing::info!("Memulai aplikasi task-tracker-api");

    let app = Router::new()
        .route("/health", get(health))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &http::Request<axum::body::Body>| {
                    let req_id = Uuid::new_v4();
                    tracing::info_span!(
                        "request",
                        method = %req.method(),
                        path = %req.uri().path(),
                        req_id = %req_id
                    )
                }) 
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
