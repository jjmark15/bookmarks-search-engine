use std::path::{Path, PathBuf};

use crate::domain::bookmark::{Bookmark, BookmarkRepository, BookmarkRepositoryError};
use crate::ports::persistence::file_system::persistence_bookmark::PersistenceBookmark;

pub(crate) struct FileSystemBookmarkRepositoryAdapter {
    store_file_path: PathBuf,
}

impl FileSystemBookmarkRepositoryAdapter {
    pub(crate) fn new<P: AsRef<Path>>(store_file_path: P) -> Self {
        FileSystemBookmarkRepositoryAdapter {
            store_file_path: store_file_path.as_ref().to_path_buf(),
        }
    }
}

impl BookmarkRepository for FileSystemBookmarkRepositoryAdapter {
    fn get_all(&self) -> Result<Vec<Bookmark>, BookmarkRepositoryError> {
        let content = std::fs::read(self.store_file_path.as_path())
            .map_err(|_| BookmarkRepositoryError::ConfigError)?;
        let data: Vec<Bookmark> =
            serde_yaml::from_slice::<Vec<PersistenceBookmark>>(content.as_slice())
                .map_err(|_| BookmarkRepositoryError::DeserializeError)?
                .iter()
                .map(Bookmark::from)
                .collect();
        Ok(data)
    }
}
