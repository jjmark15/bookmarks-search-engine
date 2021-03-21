use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use warp::http::{Response, StatusCode, Uri};
use warp::{Filter, Reply};

use crate::application::ApplicationService;
use crate::ports::http::warp::disable_caching::disable_caching;
use crate::ports::http::warp::search_error_handling::handle_search_error;
use crate::ports::http::warp::with_application_service;

pub(crate) fn bookmarks_search_filter<AS>(
    application_service: Arc<AS>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    AS: ApplicationService + Send + Sync,
{
    warp::get()
        .and(warp::query::<HashMap<String, String>>())
        .and(with_application_service(application_service))
        .map(handler)
        .map(disable_caching)
}

fn handler<AS: ApplicationService>(
    p: HashMap<String, String>,
    application_service: Arc<AS>,
) -> warp::reply::Response {
    match p.get("q") {
        Some(term) => match application_service.search(term.clone()) {
            Ok(urls) => {
                if urls.len() == 1 {
                    let url = urls.get(0).unwrap().clone();
                    return warp::redirect(Uri::from_str(url.as_str()).unwrap()).into_response();
                }

                Response::builder()
                    .body(
                        urls.iter()
                            .map(|url| url.as_str().to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                    )
                    .into_response()
            }
            Err(err) => handle_search_error(&err),
        },
        None => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(String::from("No \"q\" param in query."))
            .into_response(),
    }
}
