use uuid::Uuid;

use crate::domain::bookmark::Bookmark;

pub(crate) trait BookmarkSearchEngine {
    fn search(&self, term: String) -> Result<Vec<Bookmark>, BookmarkSearchEngineError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum BookmarkSearchEngineError {
    #[error("Invalid query was made to search engine")]
    InvalidQuery,
    #[error("Could not find Bookmark with id: {0}")]
    BookmarkNotFound(Uuid),
    #[error("Unexpected search engine error occurred: {0}")]
    Unexpected(String)
}
