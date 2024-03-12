use std::collections::HashMap;
use anyhow::{anyhow, Error};
use log::{debug, error, trace, warn};
use std::fs::File;
use std::os::windows::fs::MetadataExt;
use std::path::{PathBuf};
use std::sync::Arc;
use tantivy::schema::Schema;
use tantivy::{doc, Document, IndexWriter, Term};
use crate::conversion::Conversion;
use crate::conversion::convert_to_clear_text_strategy::MimeType;
use crate::conversion::determine_file_type::DetermineFileTypeStrategy;

use crate::file_index::persist_metadata_strategy::{FileMetadata, PersistMetadataStrategy};
use crate::file_indexer::ScannedFile;

pub(crate) trait IndexFileStrategy: Send + Sync {
    fn index_files(
        self: &mut Self,
        scanned_file: Vec<ScannedFile>,
    ) -> Result<Vec<ScannedFile>, Error>;
}

pub(crate) struct TantivyIndexStrategy {
    schema: Schema,
    index_writer: IndexWriter,
    persist_metadata_strategy: Arc<dyn PersistMetadataStrategy>,
    determine_file_type_strategy: Arc<dyn DetermineFileTypeStrategy>,
    conversion_map: HashMap<MimeType, Arc<dyn Conversion>>
}

impl TantivyIndexStrategy {
    pub(crate) fn new(
        schema: Schema,
        index_writer: IndexWriter,
        persist_metadata_strategy: Arc<impl PersistMetadataStrategy + 'static>,
        determine_file_type_strategy: Arc<dyn DetermineFileTypeStrategy>,
        conversion_map: HashMap<MimeType, Arc<dyn Conversion>>
    ) -> Self {
        TantivyIndexStrategy {
            schema,
            index_writer,
            persist_metadata_strategy,
            determine_file_type_strategy,
            conversion_map
        }
    }

    fn create_doc_from_scanned_file(&self, scanned_file: &ScannedFile) -> Result<Document, Error> {
        let contents_field = self.schema.get_field("contents")?;
        let path_field = self.schema.get_field("path")?;
        let hash_field = self.schema.get_field("hash")?;

        let mime_type = match self.determine_file_type_strategy.determine(&scanned_file) {
            Some(v) => {
                trace!("The file at path {} was found to be of type {:?}.", scanned_file.path, v);
                v
            },
            None => {
                return Err(anyhow!("The type of the file at path {} couldn't be determined.", scanned_file.path));
            }
        };

        let conversion_strategy = match self.conversion_map.get(&mime_type)
            .ok_or(anyhow!("There was an error while trying to get the conversion strategy for the given file type"))
        {
            Ok(v) => v,
            Err(e) => {
                return Err(
                    anyhow!(
                        "There was an error while trying to get a conversion strategy for given file with path {:?}. {:?}",
                        scanned_file.path,
                        e
                    )
                );
            }
        };

        let file_path = PathBuf::from(&scanned_file.path);
        return match conversion_strategy.convert(&file_path) {
            Ok(v) => {
                Ok(doc!(
                    contents_field => v,
                    path_field => (&scanned_file).path.clone(),
                    hash_field => (&scanned_file).hash.clone()
                ))
            },
            Err(e) => {
                Err(
                    anyhow!(
                        "There was an error while trying to convert the file at path {:?}. The file was found to of type {:?}. {:?}",
                        scanned_file.path,
                        mime_type,
                        e
                    )
                )
            }
        };
    }

    fn create_metadata_from_scanned_file(&self, scanned_file: &ScannedFile) -> Result<FileMetadata, Error> {
        let file_path = PathBuf::from(&scanned_file.path);

        let file = match File::open(&file_path) {
            Ok(v) => v,
            Err(e) => {
                return Err(anyhow!("There was an error while trying to open the file. {:?}", e));
            }
        };

        let os_metadata = match file.metadata() {
            Ok(v) => v,
            Err(e) => {
                return Err(anyhow!("There was an error while trying to get the metadata for the file at path {}: {:?}", scanned_file.path, e));
            }
        };

        return Ok(FileMetadata {
            path: scanned_file.path.clone(),
            size: os_metadata.file_size(),
            indexed_at: chrono::offset::Local::now(),
            hash: scanned_file.hash.clone(),
        });
    }
}

impl IndexFileStrategy for TantivyIndexStrategy {
    fn index_files(&mut self, scanned_files: Vec<ScannedFile>) -> Result<Vec<ScannedFile>, Error> {
        let path_field = self.schema.get_field("path")?;


        let mut failed_files = Vec::new();
        for scanned_file in scanned_files {

            let doc = match self.create_doc_from_scanned_file(&scanned_file) {
                Ok(v) => v,
                Err(e) => {
                    warn!("There was an error while trying to build the document from the scanned file: {:?}", e);
                    failed_files.push(scanned_file);
                    continue;
                }
            };

            let metadata = match self.create_metadata_from_scanned_file(&scanned_file) {
                Ok(v) => v,
                Err(e) => {
                    warn!("There was an error while trying to create the file metadata from the scanned file: {:?}", e);
                    failed_files.push(scanned_file);
                    continue;
                }
            };

            match self.index_writer.add_document(doc) {
                Err(e) => {
                    error!(
                        "There was an error while trying to add a document to the index {}",
                        e
                    );
                    failed_files.push(scanned_file);
                    continue;
                }
                Ok(v) => {
                    debug!("Successfully added the document {}", v);
                    v
                }
            };

            match self.persist_metadata_strategy.persist_metadata(metadata) {
                Ok(_) => {
                    debug!("Successfully persisted the metadata")
                }
                Err(e) => {
                    error!(
                        "There was an error while trying to persist the metadata: {}",
                        e
                    );
                    let delete_term = Term::from_field_text(path_field, &scanned_file.path.clone());
                    self.index_writer.delete_term(delete_term);
                    failed_files.push(scanned_file);
                }
            };
        }

        self.index_writer.commit()?;

        return Ok(failed_files); // TODO: Do something with the failed files.
    }
}
