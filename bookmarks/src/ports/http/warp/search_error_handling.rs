use warp::http::header::CACHE_CONTROL;
use warp::http::Response;
use warp::http::StatusCode;
use warp::Reply;

use crate::application::ApplicationServiceError;
use crate::domain::bookmark::BookmarkSearchEngineError;

pub(crate) fn handle_search_error(err: &ApplicationServiceError) -> warp::reply::Response {
    match err {
        ApplicationServiceError::Search(cause) => match cause {
            BookmarkSearchEngineError::InvalidQuery => Response::builder()
                .header(CACHE_CONTROL, "no-store")
                .status(StatusCode::BAD_REQUEST)
                .body(format!("{}", err))
                .into_response(),
            BookmarkSearchEngineError::BookmarkNotFound(_) => Response::builder()
                .header(CACHE_CONTROL, "no-store")
                .status(StatusCode::NOT_FOUND)
                .body(format!("{}", err))
                .into_response(),
            BookmarkSearchEngineError::Unexpected(_) => Response::builder()
                .header(CACHE_CONTROL, "no-store")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{}", err))
                .into_response(),
        },
    }
}
