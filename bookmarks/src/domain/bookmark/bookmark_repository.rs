use crate::domain::bookmark::Bookmark;

pub(crate) trait BookmarkRepository {
    fn get_all(&self) -> Result<Vec<Bookmark>, BookmarkRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum BookmarkRepositoryError {
    #[error("Could not deserialize Bookmark")]
    DeserializeError,
    #[error("Error with repository configuration")]
    ConfigError,
}
