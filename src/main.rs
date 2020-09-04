mod app;
mod config;
mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::api();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
