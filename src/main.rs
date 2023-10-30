use warp::Filter;

mod car;
mod web;

#[tokio::main]
async fn main() {
    let public = warp::get().and(
        warp::path::end()
            .and(warp::fs::file("resources/index.html"))
            .or(warp::fs::dir("resources")),
    );

    let routes = public;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
