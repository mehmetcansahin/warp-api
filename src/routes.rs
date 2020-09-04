use crate::app;
use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let home = warp::path::end()
        .and(warp::get())
        .and_then(app::http::controllers::home::index);
    let hello = warp::path!("hello")
        .and(warp::get())
        .and_then(app::http::controllers::hello::index);
    home.or(hello)
}
