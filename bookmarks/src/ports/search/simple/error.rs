#[derive(Debug, thiserror::Error)]
pub(crate) enum SimpleBookmarkSearchEngineError {
    #[error("Failed to read config file")]
    ReadConfigFile(#[from] std::io::Error),
    #[error("Failed to parse config content")]
    ParseConfigContent(#[from] serde_yaml::Error),
}
