use url::Url;

use crate::domain::bookmark::BookmarkSearchEngine;

pub(crate) trait ApplicationService {
    fn search(&self, term: String) -> Vec<Url>;
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
    fn search(&self, term: String) -> Vec<Url> {
        self.bookmark_search_engine
            .search(term)
            .iter()
            .map(|bookmark| bookmark.url().clone())
            .collect()
    }
}
