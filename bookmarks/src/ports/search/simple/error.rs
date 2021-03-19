use crate::domain::bookmark::BookmarkRepositoryError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum SimpleBookmarkSearchEngineError {
    #[error("Failed to get bookmarks from persistence")]
    PersistenceError(#[from] BookmarkRepositoryError),
    #[error("Failed to parse config content")]
    ParseConfigContent(#[from] serde_yaml::Error),
}
