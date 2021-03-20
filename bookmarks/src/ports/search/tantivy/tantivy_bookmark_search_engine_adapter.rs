use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, QueryParserError};
use tantivy::schema::{Field, Schema, STORED, TEXT};
use tantivy::{Document, Index, IndexReader, IndexWriter, ReloadPolicy, TantivyError};
use uuid::Uuid;

use crate::domain::bookmark::{
    Bookmark, BookmarkRepository, BookmarkRepositoryError, BookmarkSearchEngine,
    BookmarkSearchEngineError,
};

pub(crate) struct TantivyBookmarkSearchEngineAdapter<BR: BookmarkRepository> {
    bookmark_repository: BR,
    initialised_engine: Option<InitialisedEngine>,
}

impl<BR: BookmarkRepository> TantivyBookmarkSearchEngineAdapter<BR> {
    pub(crate) fn new(bookmark_repository: BR) -> Self {
        TantivyBookmarkSearchEngineAdapter {
            bookmark_repository,
            initialised_engine: None,
        }
    }

    fn schema() -> Schema {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("id", STORED);
        schema_builder.add_text_field("name", TEXT);
        schema_builder.add_text_field("description", TEXT);
        schema_builder.add_text_field("tags", TEXT);

        schema_builder.build()
    }

    fn add_document(
        index_writer: &mut IndexWriter,
        bookmark_document_fields: &BookmarkDocumentFields,
        bookmark: &Bookmark,
    ) {
        let mut bookmark_doc = Document::default();
        bookmark_doc.add_text(bookmark_document_fields.id, bookmark.id());
        bookmark_doc.add_text(bookmark_document_fields.name, bookmark.name());
        if let Some(description) = bookmark.description() {
            bookmark_doc.add_text(bookmark_document_fields.description, description);
        }
        bookmark
            .tags()
            .iter()
            .for_each(|tag| bookmark_doc.add_text(bookmark_document_fields.tags, tag.to_string()));
        index_writer.add_document(bookmark_doc);
    }

    pub(crate) fn initialise(
        &mut self,
        bookmarks: Vec<Bookmark>,
    ) -> Result<(), TantivyBookmarkSearchEngineAdapterError> {
        let schema = Self::schema();
        let index = Index::create_in_ram(schema.clone());
        let mut index_writer = index.writer(50_000_000).unwrap();

        let document_fields: BookmarkDocumentFields = BookmarkDocumentFields::from(&schema);

        bookmarks.iter().for_each(|bookmark| {
            Self::add_document(&mut index_writer, &document_fields, bookmark);
        });

        index_writer.commit()?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();

        self.initialised_engine = Some(InitialisedEngine {
            reader,
            index,
            document_fields,
        });

        Ok(())
    }

    fn initialised_engine(&self) -> &InitialisedEngine {
        &self
            .initialised_engine
            .as_ref()
            .ok_or(TantivyBookmarkSearchEngineAdapterError::NotInitialised)
            .unwrap()
    }
}

impl<BR: BookmarkRepository> BookmarkSearchEngine for TantivyBookmarkSearchEngineAdapter<BR> {
    fn search(&self, term: String) -> Result<Vec<Bookmark>, BookmarkSearchEngineError> {
        let initialised_engine = self.initialised_engine();
        let reader = &initialised_engine.reader;
        let index = &initialised_engine.index;
        let document_fields = &initialised_engine.document_fields;
        let search_fields = vec![
            document_fields.name,
            document_fields.description,
            document_fields.description,
            document_fields.tags,
        ];

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, search_fields);
        let query = query_parser
            .parse_query(term.as_str())
            .map_err(TantivyBookmarkSearchEngineAdapterError::from)?;
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(10))
            .map_err(TantivyBookmarkSearchEngineAdapterError::from)?;

        let ids: Vec<Uuid> = top_docs
            .iter()
            .map(|(_score, doc_address)| {
                let retrieved_doc = searcher
                    .doc(*doc_address)
                    .map_err(TantivyBookmarkSearchEngineAdapterError::from)
                    .unwrap();
                let id = retrieved_doc
                    .field_values()
                    .iter()
                    .find_map(|val| Uuid::parse_str(val.value().text().unwrap()).ok());

                id.ok_or(TantivyBookmarkSearchEngineAdapterError::MissingRequiredField)
                    .unwrap()
            })
            .collect();

        Ok(ids
            .iter()
            .map(|id| {
                self.bookmark_repository
                    .get(*id)
                    .map_err(TantivyBookmarkSearchEngineAdapterError::from)
            })
            .collect::<Result<Vec<Bookmark>, TantivyBookmarkSearchEngineAdapterError>>()?)
    }
}

struct InitialisedEngine {
    reader: IndexReader,
    index: Index,
    document_fields: BookmarkDocumentFields,
}

struct BookmarkDocumentFields {
    id: Field,
    name: Field,
    description: Field,
    tags: Field,
}

impl BookmarkDocumentFields {
    fn new(id: Field, name: Field, description: Field, tags: Field) -> Self {
        BookmarkDocumentFields {
            id,
            name,
            description,
            tags,
        }
    }
}

impl From<&Schema> for BookmarkDocumentFields {
    fn from(schema: &Schema) -> Self {
        BookmarkDocumentFields::new(
            schema.get_field("id").unwrap(),
            schema.get_field("name").unwrap(),
            schema.get_field("description").unwrap(),
            schema.get_field("tags").unwrap(),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum TantivyBookmarkSearchEngineAdapterError {
    #[error("Search engine has not been initialised")]
    NotInitialised,
    #[error(transparent)]
    Tantivy(#[from] TantivyError),
    #[error(transparent)]
    QueryParse(#[from] QueryParserError),
    #[error("Retrieved document is missing a required field")]
    MissingRequiredField,
    #[error(transparent)]
    BookmarkNotFound(BookmarkRepositoryError),
    #[error(transparent)]
    BookmarkRepository(BookmarkRepositoryError),
}

impl From<TantivyBookmarkSearchEngineAdapterError> for BookmarkSearchEngineError {
    fn from(err: TantivyBookmarkSearchEngineAdapterError) -> Self {
        match err {
            TantivyBookmarkSearchEngineAdapterError::NotInitialised
            | TantivyBookmarkSearchEngineAdapterError::MissingRequiredField
            | TantivyBookmarkSearchEngineAdapterError::BookmarkNotFound(
                BookmarkRepositoryError::Unexpected(_),
            )
            | TantivyBookmarkSearchEngineAdapterError::Tantivy(_)
            | TantivyBookmarkSearchEngineAdapterError::BookmarkRepository(_) => {
                BookmarkSearchEngineError::Unexpected(format!("{}", err))
            }
            TantivyBookmarkSearchEngineAdapterError::QueryParse(_) => {
                BookmarkSearchEngineError::InvalidQuery
            }
            TantivyBookmarkSearchEngineAdapterError::BookmarkNotFound(
                BookmarkRepositoryError::BookmarkNotFound(id),
            ) => BookmarkSearchEngineError::BookmarkNotFound(id),
        }
    }
}

impl From<BookmarkRepositoryError> for TantivyBookmarkSearchEngineAdapterError {
    fn from(err: BookmarkRepositoryError) -> Self {
        match err {
            BookmarkRepositoryError::BookmarkNotFound(_) => {
                TantivyBookmarkSearchEngineAdapterError::BookmarkNotFound(err)
            }
            BookmarkRepositoryError::Unexpected(_) => {
                TantivyBookmarkSearchEngineAdapterError::BookmarkRepository(err)
            }
        }
    }
}
