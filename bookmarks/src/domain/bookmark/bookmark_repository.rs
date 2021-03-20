use uuid::Uuid;

use crate::domain::bookmark::Bookmark;

pub(crate) trait BookmarkRepository {
    fn get_all(&self) -> Result<Vec<Bookmark>, BookmarkRepositoryError>;

    fn get(&self, id: Uuid) -> Result<Bookmark, BookmarkRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum BookmarkRepositoryError {
    #[error("Could not find Bookmark with id: {0}")]
    BookmarkNotFound(Uuid),
    #[error("Unexpected persistence error occurred: {0}")]
    Unexpected(String),
}
