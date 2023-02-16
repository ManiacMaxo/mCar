use std::sync::{Arc, Mutex};
use warp::Filter;

mod camera;
mod car;
mod web;

use camera::Camera;
use car::Car;

#[tokio::main]
async fn main() {
    let car = Arc::new(Car::new());

    let control = warp::path("control")
        .and(warp::ws())
        .and(warp::any().map(move || car.clone()))
        .map(|ws: warp::ws::Ws, car: Arc<Car>| {
            ws.on_upgrade(move |socket| web::handle_socket(socket, car))
        });

    let public = warp::get().and(
        warp::path::end()
            .and(warp::fs::file("resources/index.html"))
            .or(warp::fs::dir("resources"))
            .with(warp::compression::gzip()),
    );

    let camera = Arc::new(Camera::new());
    camera.start().expect("could not start camera");

    let feed = warp::path!("feed")
        .and(warp::get())
        .and(warp::any().map(move || camera.clone()))
        .map(web::camera_feed);

    let api = warp::path!("api").and(feed);

    let routes = public.or(api).or(control);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
