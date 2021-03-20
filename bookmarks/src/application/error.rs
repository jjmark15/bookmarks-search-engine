use crate::domain::bookmark::BookmarkSearchEngineError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ApplicationServiceError {
    #[error(transparent)]
    Search(#[from] BookmarkSearchEngineError),
}
