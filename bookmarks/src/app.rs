use std::path::PathBuf;
use std::sync::Arc;

use warp::Filter;

use crate::application::{ApplicationService, ApplicationServiceImpl};
use crate::domain::bookmark::BookmarkFactoryImpl;
use crate::ports::http::warp::{bookmarks_search_filter, bookmarks_suggestions_filter};
use crate::ports::persistence::file_system::FileSystemBookmarkRepositoryAdapter;
use crate::ports::search::simple::{
    SimpleBookmarkSearchEngine, SimpleBookmarkSearchEngineInitialisationError,
};

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

    pub async fn run(&self) -> Result<(), AppInitialisationError> {
        let application_service = ApplicationServiceImpl::new(
            self.bookmark_search_engine()
                .map_err(map_initialisation_error_cause)?,
        );

        warp::serve(self.routes(Arc::new(application_service)))
            .run(([127, 0, 0, 1], 3033))
            .await;

        Ok(())
    }

    fn bookmark_search_engine(
        &self,
    ) -> Result<SimpleBookmarkSearchEngine, SimpleBookmarkSearchEngineInitialisationError> {
        let bookmark_factory = BookmarkFactoryImpl::new();
        let bookmark_repository = FileSystemBookmarkRepositoryAdapter::new(
            self.search_engine_config_path.as_path(),
            bookmark_factory,
        );
        SimpleBookmarkSearchEngine::new(bookmark_repository)
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
#[error("App failed to start: {cause}")]
pub struct AppInitialisationError {
    cause: AppInitialisationCause,
}

impl AppInitialisationError {
    fn new(cause: AppInitialisationCause) -> Self {
        AppInitialisationError { cause }
    }
}

#[derive(Debug, thiserror::Error)]
enum AppInitialisationCause {
    #[error(transparent)]
    SearchEngine(#[from] SimpleBookmarkSearchEngineInitialisationError),
}

fn map_initialisation_error_cause<C: Into<AppInitialisationCause>>(
    cause: C,
) -> AppInitialisationError {
    AppInitialisationError::new(cause.into())
}
