use url::Url;
use uuid::Uuid;

use crate::domain::bookmark::Bookmark;

pub(crate) trait BookmarkFactory {
    fn create(
        &self,
        url: Url,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Bookmark;
}

pub(crate) struct BookmarkFactoryImpl;

impl BookmarkFactoryImpl {
    pub(crate) fn new() -> Self {
        BookmarkFactoryImpl
    }
}

impl BookmarkFactory for BookmarkFactoryImpl {
    fn create(
        &self,
        url: Url,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Bookmark {
        Bookmark::new(Uuid::new_v4(), url, name, description, tags)
    }
}
