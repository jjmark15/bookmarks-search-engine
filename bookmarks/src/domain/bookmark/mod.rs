use url::Url;
use uuid::Uuid;

pub(crate) use bookmark_factory::*;
pub(crate) use bookmark_repository::*;
pub(crate) use bookmark_search_engine::*;

mod bookmark_factory;
mod bookmark_repository;
mod bookmark_search_engine;

pub(crate) struct Bookmark {
    id: Uuid,
    url: Url,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
}

impl Bookmark {
    pub(crate) fn new(
        id: Uuid,
        url: Url,
        name: String,
        description: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        Bookmark {
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
