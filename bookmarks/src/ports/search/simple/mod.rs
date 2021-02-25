pub(crate) use config_reader::*;
pub(crate) use error::*;

use crate::domain::bookmark::{Bookmark as DomainBookmark, BookmarkSearchEngine};
use crate::ports::search::simple::bookmark::Bookmark;

mod bookmark;
mod config_reader;
mod error;

pub(crate) struct SimpleBookmarkSearchEngine {
    data: Vec<Bookmark>,
}

impl SimpleBookmarkSearchEngine {
    pub(crate) fn new<CR: ConfigReader>(
        config_reader: CR,
    ) -> Result<Self, SimpleBookmarkSearchEngineError> {
        let data = config_reader.read()?;

        Ok(SimpleBookmarkSearchEngine { data })
    }
}

impl BookmarkSearchEngine for SimpleBookmarkSearchEngine {
    fn search(&self, term: String) -> Vec<DomainBookmark> {
        let lowercase_term = term.to_lowercase();
        self.data
            .iter()
            .filter(|bookmark| bookmark.name().to_lowercase().contains(&lowercase_term))
            .map(|bookmark| bookmark.clone().into())
            .collect()
    }
}
