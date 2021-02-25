use crate::domain::bookmark::Bookmark;

pub(crate) trait BookmarkSearchEngine {
    fn search(&self, term: String) -> Vec<Bookmark>;
}
