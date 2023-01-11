use warp::Filter;

pub mod web;

#[tokio::main]
async fn main() {
    // let car = Car();

    let control = warp::path("control")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| web::handle_socket(socket)));

    let index = warp::path::end().and(warp::fs::file("client/dist/index.html"));
    let public = index
        .or(warp::fs::dir("./client/dist/"))
        .with(warp::compression::gzip());

    let routes = public.or(control);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
