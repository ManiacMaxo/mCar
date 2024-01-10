use axum::{routing::get, Router};

mod car;
mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/control", get(web::ws_handler))
        .route("/api/feed", get(web::cam_feed))
        .fallback(web::static_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
