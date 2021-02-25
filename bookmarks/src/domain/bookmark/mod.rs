use url::Url;

pub(crate) use bookmark_search_engine::*;

mod bookmark_search_engine;

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
