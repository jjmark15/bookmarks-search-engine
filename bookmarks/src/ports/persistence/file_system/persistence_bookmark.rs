use url::Url;

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
        self.description.as_ref()
    }

    pub(crate) fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}
