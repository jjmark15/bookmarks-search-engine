use std::collections::HashMap;
use std::sync::Arc;

use warp::http::{Response, StatusCode};
use warp::{Filter, Reply};

use crate::application::ApplicationService;
use crate::ports::http::warp::disable_caching::disable_caching;
use crate::ports::http::warp::search_error_handling::handle_search_error;
use crate::ports::http::warp::with_application_service;

pub(crate) fn bookmarks_suggestions_filter<AS>(
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
        Some(term) => match application_service.suggest(term.clone()) {
            Ok(names) => {
                let body = SuggestionResponse::new(term.clone(), names);

                Response::builder()
                    .status(StatusCode::OK)
                    .header(
                        warp::http::header::CONTENT_TYPE,
                        "application/x-suggestions+json",
                    )
                    .body(serde_json::to_string(&body).unwrap())
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

#[derive(Debug, Clone, serde::Serialize)]
#[serde(into = "SuggestionResponseBody")]
struct SuggestionResponse {
    query: String,
    suggestions: Vec<String>,
}

impl SuggestionResponse {
    fn new(query: String, suggestions: Vec<String>) -> Self {
        SuggestionResponse { query, suggestions }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
enum StringOrStrings {
    String(String),
    Strings(Vec<String>),
}

#[derive(Debug, serde::Serialize)]
#[serde(transparent)]
struct SuggestionResponseBody {
    inner: Vec<StringOrStrings>,
}

impl From<SuggestionResponse> for SuggestionResponseBody {
    fn from(response: SuggestionResponse) -> Self {
        let mut inner = vec![];
        inner.push(StringOrStrings::String(response.query));
        inner.push(StringOrStrings::Strings(response.suggestions));

        SuggestionResponseBody { inner }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn returns_open_search_compatible_response() {
        let suggestion_response =
            SuggestionResponse::new("query".to_string(), vec!["suggestion1".to_string()]);

        let serialized = serde_json::to_string(&suggestion_response).unwrap();

        assert_that(&serialized).is_equal_to(&"[\"query\",[\"suggestion1\"]]".to_string());
    }
}
