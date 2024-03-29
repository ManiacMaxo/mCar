use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};

mod camera;
mod car;
mod web;

#[tokio::main]
async fn main() {
    let cam = Arc::new(camera::Camera::new(0));
    cam.start();

    let car = Arc::new(Mutex::new(car::Car::new()));

    let app = Router::new()
        .route("/control", get(web::ws_handler).with_state(car))
        .route("/api/feed", get(web::cam_feed).with_state(cam))
        .fallback(web::static_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
