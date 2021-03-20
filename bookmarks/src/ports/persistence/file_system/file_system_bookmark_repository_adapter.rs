use std::collections::HashMap;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::domain::bookmark::{Bookmark, BookmarkRepository, BookmarkRepositoryError};
use crate::ports::persistence::file_system::memory_bookmark::MemoryBookmark;
use crate::ports::persistence::file_system::persistence_bookmark::PersistenceBookmark;

pub(crate) struct FileSystemBookmarkRepositoryAdapter {
    store_file_path: PathBuf,
    inner: Option<HashMap<Uuid, MemoryBookmark>>,
}

impl FileSystemBookmarkRepositoryAdapter {
    pub(crate) fn new<P: AsRef<Path>>(store_file_path: P) -> Self {
        FileSystemBookmarkRepositoryAdapter {
            store_file_path: store_file_path.as_ref().to_path_buf(),
            inner: None,
        }
    }

    fn inner(
        &self,
    ) -> Result<&HashMap<Uuid, MemoryBookmark>, FileSystemBookmarkRepositoryAdapterError> {
        self.inner
            .as_ref()
            .ok_or(FileSystemBookmarkRepositoryAdapterError::NotInitialised)
    }

    pub(crate) fn initialise(&mut self) -> Result<(), FileSystemBookmarkRepositoryAdapterError> {
        let content = std::fs::read(self.store_file_path.as_path())
            .map_err(|_| FileSystemBookmarkRepositoryAdapterError::ConfigError)?;
        let data: HashMap<Uuid, MemoryBookmark> =
            serde_yaml::from_slice::<Vec<PersistenceBookmark>>(content.as_slice())
                .map_err(|_| FileSystemBookmarkRepositoryAdapterError::DeserializeError)?
                .iter()
                .map(from_persistence_bookmark)
                .map(|bookmark| (bookmark.id(), bookmark))
                .collect();
        self.inner = Some(data);
        Ok(())
    }
}

impl BookmarkRepository for FileSystemBookmarkRepositoryAdapter {
    fn get_all(&self) -> Result<Vec<Bookmark>, BookmarkRepositoryError> {
        Ok(self.inner()?.values().map(Bookmark::from).collect())
    }

    fn get(&self, id: Uuid) -> Result<Bookmark, BookmarkRepositoryError> {
        let bookmark = self
            .inner()?
            .get(&id)
            .ok_or(BookmarkRepositoryError::BookmarkNotFound(id))?;
        Ok(bookmark.into())
    }
}

fn from_persistence_bookmark(bookmark: &PersistenceBookmark) -> MemoryBookmark {
    MemoryBookmark::new(
        Uuid::new_v4(),
        bookmark.url().clone(),
        bookmark.name().clone(),
        bookmark.description().cloned(),
        bookmark.tags().clone(),
    )
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum FileSystemBookmarkRepositoryAdapterError {
    #[error("Could not deserialize Bookmark")]
    DeserializeError,
    #[error("Error with repository configuration")]
    ConfigError,
    #[error("Repository has not been initialised")]
    NotInitialised,
}

impl From<FileSystemBookmarkRepositoryAdapterError> for BookmarkRepositoryError {
    fn from(err: FileSystemBookmarkRepositoryAdapterError) -> Self {
        match err {
            FileSystemBookmarkRepositoryAdapterError::DeserializeError
            | FileSystemBookmarkRepositoryAdapterError::NotInitialised
            | FileSystemBookmarkRepositoryAdapterError::ConfigError => {
                BookmarkRepositoryError::Unexpected(format!("{}", err))
            }
        }
    }
}
