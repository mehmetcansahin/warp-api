use serde_json::json;
use warp::http::StatusCode;

pub async fn index() -> Result<impl warp::Reply, warp::Rejection> {
    let response = json!({
        "message": "Hello, World!"
    });
    let json = warp::reply::json(&response);
    Ok(warp::reply::with_status(json, StatusCode::OK))
}
