use url::Url;
use uuid::Uuid;

use crate::domain::bookmark::Bookmark;

#[derive(Debug, serde::Deserialize)]
pub(super) struct MemoryBookmark {
    id: Uuid,
    url: Url,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
}

impl MemoryBookmark {
    pub(crate) fn new(
        id: Uuid,
        url: Url,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        MemoryBookmark {
            id,
            url,
            name,
            description,
            tags,
        }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub(crate) fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}

impl From<&MemoryBookmark> for Bookmark {
    fn from(bookmark: &MemoryBookmark) -> Self {
        Bookmark::new(
            bookmark.id(),
            bookmark.url().clone(),
            bookmark.name().clone(),
            bookmark.description().cloned(),
            bookmark.tags().clone(),
        )
    }
}
