use std::path::{Path, PathBuf};

use crate::domain::bookmark::{
    Bookmark, BookmarkFactory, BookmarkRepository, BookmarkRepositoryError,
};
use crate::ports::persistence::file_system::persistence_bookmark::PersistenceBookmark;

pub(crate) struct FileSystemBookmarkRepositoryAdapter<BF: BookmarkFactory> {
    store_file_path: PathBuf,
    bookmark_factory: BF,
}

impl<BF: BookmarkFactory> FileSystemBookmarkRepositoryAdapter<BF> {
    pub(crate) fn new<P: AsRef<Path>>(store_file_path: P, bookmark_factory: BF) -> Self {
        FileSystemBookmarkRepositoryAdapter {
            store_file_path: store_file_path.as_ref().to_path_buf(),
            bookmark_factory,
        }
    }
}

impl<BF: BookmarkFactory> BookmarkRepository for FileSystemBookmarkRepositoryAdapter<BF> {
    fn get_all(&self) -> Result<Vec<Bookmark>, BookmarkRepositoryError> {
        let content = std::fs::read(self.store_file_path.as_path())
            .map_err(|_| BookmarkRepositoryError::ConfigError)?;
        let data: Vec<Bookmark> =
            serde_yaml::from_slice::<Vec<PersistenceBookmark>>(content.as_slice())
                .map_err(|_| BookmarkRepositoryError::DeserializeError)?
                .iter()
                .map(|bookmark| from_persistence_bookmark(&self.bookmark_factory, bookmark))
                .collect();
        Ok(data)
    }
}

fn from_persistence_bookmark<BF: BookmarkFactory>(
    bookmark_factory: &BF,
    bookmark: &PersistenceBookmark,
) -> Bookmark {
    bookmark_factory.create(
        bookmark.url().clone(),
        bookmark.name().to_string(),
        bookmark.description().map(ToString::to_string),
        bookmark.tags().clone(),
    )
}
