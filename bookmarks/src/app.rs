use std::path::PathBuf;
use std::sync::Arc;

use warp::Filter;

use crate::application::{ApplicationService, ApplicationServiceImpl};
use crate::ports::http::warp::{bookmarks_search_filter, bookmarks_suggestions_filter};
use crate::ports::search::simple::{FileConfigReader, SimpleBookmarkSearchEngine};

#[derive(Default)]
pub struct App {
    search_engine_config_path: PathBuf,
}

impl App {
    pub fn new(search_engine_config_path: PathBuf) -> Self {
        App {
            search_engine_config_path,
        }
    }

    pub async fn run(&self) -> Result<(), AppError> {
        let application_service = ApplicationServiceImpl::new(self.bookmark_search_engine()?);

        warp::serve(self.routes(Arc::new(application_service)))
            .run(([127, 0, 0, 1], 3033))
            .await;

        Ok(())
    }

    fn bookmark_search_engine(&self) -> Result<SimpleBookmarkSearchEngine, AppError> {
        let config_reader = FileConfigReader::new(self.search_engine_config_path.as_path());
        Ok(SimpleBookmarkSearchEngine::new(config_reader)
            .map_err(|err| AppError::BookmarkSearchEngineError(format!("{}", err)))?)
    }

    fn routes<AS>(
        &self,
        application_service: Arc<AS>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    where
        AS: ApplicationService + Send + Sync,
    {
        let search = warp::path("search").and(bookmarks_search_filter(application_service.clone()));
        let suggestions =
            warp::path("suggestions").and(bookmarks_suggestions_filter(application_service));

        warp::any().and(search.or(suggestions))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    BookmarkSearchEngineError(String),
}
