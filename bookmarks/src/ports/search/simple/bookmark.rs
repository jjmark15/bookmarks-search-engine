use url::Url;

use crate::domain::bookmark::Bookmark as DomainBookmark;

#[derive(Clone, serde::Deserialize)]
pub(crate) struct Bookmark {
    url: Url,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
}

impl Bookmark {
    pub(crate) fn new(
        url: Url,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        Bookmark {
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

impl From<DomainBookmark> for Bookmark {
    fn from(bookmark: DomainBookmark) -> Self {
        Bookmark::new(
            bookmark.url().clone(),
            bookmark.name().clone(),
            bookmark.description().cloned(),
            bookmark.tags().clone(),
        )
    }
}

impl From<Bookmark> for DomainBookmark {
    fn from(bookmark: Bookmark) -> Self {
        DomainBookmark::new(
            bookmark.url().clone(),
            bookmark.name().clone(),
            bookmark.description().cloned(),
            bookmark.tags().clone(),
        )
    }
}
