use log::error;
use std::ops::Deref;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Schema;
use tantivy::{Index, IndexReader};

pub(crate) trait SearchFileStrategy: Send + Sync {
    fn search_file(self: &Self, search_term: &str) -> Vec<String>;
}

pub(crate) struct TantivySearchStrategy {
    index: Arc<Index>,
    index_reader: IndexReader,
    schema: Schema,
}

impl TantivySearchStrategy {
    pub(crate) fn new(index: Arc<Index>, index_reader: IndexReader, schema: Schema) -> Self {
        TantivySearchStrategy {
            index,
            index_reader,
            schema,
        }
    }
}

impl SearchFileStrategy for TantivySearchStrategy {
    fn search_file(self: &Self, search_term: &str) -> Vec<String> {
        let path_field = match self.schema.get_field("path") {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "There was an error while trying to get field \"path\" from index: {:?}",
                    e
                );
                return Vec::new();
            }
        };
        let contents_field = match self.schema.get_field("contents") {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "There was an error while trying to get field \"contents\" from index: {:?}",
                    e
                );
                return Vec::new();
            }
        };
        let searcher = self.index_reader.searcher();
        let query_parser = QueryParser::for_index(self.index.deref(), vec![contents_field]);
        let query = match query_parser.parse_query(search_term) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "There was an error while trying to parse the query: {:?}",
                    e
                );
                return Vec::new();
            }
        };

        let top_docs = match searcher.search(&query, &TopDocs::with_limit(1000)) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "There was an error while trying to search with the given query: {:?}",
                    e
                );
                return Vec::new();
            }
        };

        let _offset = 50;

        return top_docs
            .into_iter()
            .map(|(_, doc_address)| { searcher.doc(doc_address) })
            .filter_map(|doc_res| {
                if doc_res.is_err() {
                    error!("There was an error while trying to get a document");
                    return None;
                }

                return Some(doc_res.unwrap());
            })
            .map(|doc| {
                // let content_slice: String = doc
                //     .get_all(contents_field)
                //     .filter_map(|field_value| {
                //         let as_string = field_value.as_text()?;
                //
                //         let index_of_search_term = as_string.find(search_term)?;
                //
                //         let slice_starting_index = index_of_search_term - 50;
                //         let slice_starting_index = if slice_starting_index > 0 {
                //             slice_starting_index
                //         } else {
                //             0
                //         };
                //
                //         let slice_ending_index = index_of_search_term + 50;
                //         let slice_ending_index = if slice_ending_index < search_term.len() - 1 {
                //             slice_ending_index
                //         } else {
                //             search_term.len() - 1
                //         };
                //
                //
                //         return Some(&as_string[slice_starting_index..slice_ending_index]);
                //     })
                //     .collect();

                return doc
                    .get_all(path_field)
                    .filter_map(|field_value| {
                        let as_text = field_value.as_text();
                        if as_text.is_none() {
                            error!("There was an error while trying to get the value of the field \"path\" for a doc.");
                            return None;
                        }

                        return as_text;
                    })
                    .collect();
            })
            .collect();
    }
}
