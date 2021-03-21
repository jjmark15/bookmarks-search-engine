use warp::http::header::CACHE_CONTROL;
use warp::Reply;

pub(super) fn disable_caching(response: warp::reply::Response) -> warp::reply::Response {
    warp::reply::with_header(response, CACHE_CONTROL, "no-store").into_response()
}
