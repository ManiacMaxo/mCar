use std::sync::Arc;
use warp::Filter;

pub mod web;
mod car;

use car::Car;

#[tokio::main]
async fn main() {
    let car_instance = Arc::new(Car::new());
    let car = warp::any().map(move || car_instance.clone());

    let control =
        warp::path("control")
            .and(warp::ws())
            .and(car)
            .map(|ws: warp::ws::Ws, car: Arc<Car>| {
                ws.on_upgrade(move |socket| web::handle_socket(socket, car))
            });

    let index = warp::path::end().and(warp::fs::file("resources/index.html"));
    let public = index
        .or(warp::fs::dir("./resources"))
        .with(warp::compression::gzip());

    let routes = public.or(control);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
