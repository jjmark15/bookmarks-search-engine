use url::Url;

use crate::domain::bookmark::Bookmark;

#[derive(Clone, serde::Deserialize)]
pub(crate) struct SimpleSearchBookmark {
    url: Url,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
}

impl SimpleSearchBookmark {
    pub(crate) fn new(
        url: Url,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        SimpleSearchBookmark {
            url,
            name,
            description,
            tags,
        }
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
            bookmark.url().clone(),
            bookmark.name().clone(),
            bookmark.description().cloned(),
            bookmark.tags().clone(),
        )
    }
}
