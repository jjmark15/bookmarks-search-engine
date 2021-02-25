use std::path::{Path, PathBuf};

use crate::ports::search::simple::bookmark::Bookmark;
use crate::ports::search::simple::SimpleBookmarkSearchEngineError;

pub(crate) trait ConfigReader {
    fn read(&self) -> Result<Vec<Bookmark>, SimpleBookmarkSearchEngineError>;
}

pub(crate) struct FileConfigReader {
    config_file_path: PathBuf,
}

impl FileConfigReader {
    pub(crate) fn new<P: AsRef<Path>>(config_file_path: P) -> Self {
        FileConfigReader {
            config_file_path: config_file_path.as_ref().to_path_buf(),
        }
    }
}

impl ConfigReader for FileConfigReader {
    fn read(&self) -> Result<Vec<Bookmark>, SimpleBookmarkSearchEngineError> {
        let content = std::fs::read(self.config_file_path.as_path())?;
        let data: Vec<Bookmark> = serde_yaml::from_slice(content.as_slice())?;
        Ok(data)
    }
}
