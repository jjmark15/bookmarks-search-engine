use url::Url;

pub(crate) use error::*;

use crate::domain::bookmark::BookmarkSearchEngine;

mod error;

pub(crate) trait ApplicationService {
    fn search(&self, term: String) -> Result<Vec<Url>, ApplicationServiceError>;

    fn suggest(&self, term: String) -> Result<Vec<String>, ApplicationServiceError>;
}

#[derive(Default)]
pub(crate) struct ApplicationServiceImpl<BSE: BookmarkSearchEngine> {
    bookmark_search_engine: BSE,
}

impl<BSE: BookmarkSearchEngine> ApplicationServiceImpl<BSE> {
    pub(crate) fn new(bookmark_search_engine: BSE) -> Self {
        ApplicationServiceImpl {
            bookmark_search_engine,
        }
    }
}

impl<BSE: BookmarkSearchEngine> ApplicationService for ApplicationServiceImpl<BSE> {
    fn search(&self, term: String) -> Result<Vec<Url>, ApplicationServiceError> {
        Ok(self
            .bookmark_search_engine
            .search(term)
            .map_err(ApplicationServiceError::from)?
            .iter()
            .map(|bookmark| bookmark.url().clone())
            .collect())
    }

    fn suggest(&self, term: String) -> Result<Vec<String>, ApplicationServiceError> {
        Ok(self
            .bookmark_search_engine
            .search(term)
            .map_err(ApplicationServiceError::from)?
            .iter()
            .map(|bookmark| bookmark.name().clone())
            .collect())
    }
}
