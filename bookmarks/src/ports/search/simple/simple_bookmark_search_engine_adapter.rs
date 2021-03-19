use crate::domain::bookmark::{Bookmark, BookmarkRepository, BookmarkSearchEngine};
use crate::ports::search::simple::bookmark::SimpleSearchBookmark;
use crate::ports::search::simple::SimpleBookmarkSearchEngineError;

pub(crate) struct SimpleBookmarkSearchEngine {
    data: Vec<SimpleSearchBookmark>,
}

impl SimpleBookmarkSearchEngine {
    pub(crate) fn new<BR: BookmarkRepository>(
        bookmark_repository: BR,
    ) -> Result<Self, SimpleBookmarkSearchEngineError> {
        let data: Vec<SimpleSearchBookmark> = bookmark_repository
            .get_all()?
            .iter()
            .map(SimpleSearchBookmark::from)
            .collect();

        Ok(SimpleBookmarkSearchEngine { data })
    }
}

impl BookmarkSearchEngine for SimpleBookmarkSearchEngine {
    fn search(&self, term: String) -> Vec<Bookmark> {
        let lowercase_term = term.to_lowercase();
        self.data
            .iter()
            .filter(|bookmark| bookmark.name().to_lowercase().contains(&lowercase_term))
            .map(|bookmark| bookmark.clone().into())
            .collect()
    }
}
