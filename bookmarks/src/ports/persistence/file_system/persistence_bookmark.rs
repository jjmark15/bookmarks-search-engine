use url::Url;

use crate::domain::bookmark::Bookmark;

#[derive(Debug, serde::Deserialize)]
pub(super) struct PersistenceBookmark {
    url: Url,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
}

impl PersistenceBookmark {
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

impl From<&PersistenceBookmark> for Bookmark {
    fn from(bookmark: &PersistenceBookmark) -> Self {
        Bookmark::new(
            bookmark.url().clone(),
            bookmark.name().to_string(),
            bookmark.description().map(String::to_string),
            bookmark.tags().clone(),
        )
    }
}
