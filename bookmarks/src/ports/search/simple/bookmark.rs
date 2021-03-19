use url::Url;
use uuid::Uuid;

use crate::domain::bookmark::Bookmark;

#[derive(Clone)]
pub(crate) struct SimpleSearchBookmark {
    id: Uuid,
    url: Url,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
}

impl SimpleSearchBookmark {
    pub(crate) fn new(
        id: Uuid,
        url: Url,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        SimpleSearchBookmark {
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
        match &self.description {
            Some(description) => Some(description),
            None => None,
        }
    }

    pub(crate) fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}

impl From<&Bookmark> for SimpleSearchBookmark {
    fn from(bookmark: &Bookmark) -> Self {
        SimpleSearchBookmark::new(
            bookmark.id(),
            bookmark.url().clone(),
            bookmark.name().clone(),
            bookmark.description().cloned(),
            bookmark.tags().clone(),
        )
    }
}

impl From<SimpleSearchBookmark> for Bookmark {
    fn from(bookmark: SimpleSearchBookmark) -> Self {
        Bookmark::new(
            bookmark.id(),
            bookmark.url().clone(),
            bookmark.name().clone(),
            bookmark.description().cloned(),
            bookmark.tags().clone(),
        )
    }
}
