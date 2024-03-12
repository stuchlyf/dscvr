use crate::file_index::index_file_strategy::IndexFileStrategy;
use crate::file_index::search_file_strategy::SearchFileStrategy;
use crate::file_indexer::file_indexer_server::FileIndexer;
use crate::file_indexer::{FindDuplicatedFilesQuery, FindDuplicatedFilesResponse, IndexFileQuery, SearchFileByContentsQuery, SearchFileResponse};
use crate::proto_utils::Empty;
use log::{debug, error, info};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use crate::file_index::find_duplicated_files_strategy::FindDuplicatedFilesStrategy;

pub(crate) mod index_file_strategy;
pub(crate) mod persist_metadata_strategy;
pub(crate) mod search_file_strategy;
pub(crate) mod find_duplicated_files_strategy;

pub(crate) struct FileIndexService {
    index_file_strategy: Arc<Mutex<dyn IndexFileStrategy>>,
    search_file_strategy: Arc<dyn SearchFileStrategy>,
    find_duplicated_files_strategy: Arc<dyn FindDuplicatedFilesStrategy>
}

impl FileIndexService {
    pub(crate) fn new(
        index_file_strategy: Arc<Mutex<impl IndexFileStrategy + 'static>>,
        search_file_strategy: Arc<impl SearchFileStrategy + 'static>,
        find_duplicated_files_strategy: Arc<impl FindDuplicatedFilesStrategy + 'static>
    ) -> Self {
        Self {
            index_file_strategy,
            search_file_strategy,
            find_duplicated_files_strategy,
        }
    }
}

#[tonic::async_trait]
impl FileIndexer for FileIndexService {
    async fn index_file(
        self: &Self,
        request: Request<IndexFileQuery>,
    ) -> Result<Response<Empty>, Status> {
        debug!("starting to process request");

        let index_file_query = request.into_inner();
        let files_to_index_count = index_file_query.scanned_files.len();
        debug!(
            "Request has {} amount of files to index",
            files_to_index_count
        );

        let mut guard = match self.index_file_strategy.lock() {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "There was an error while trying to acquire the lock of a Mutex: {:?}",
                    e
                );
                return Err(Status::internal("There was an internal server error."));
            }
        };

        let index_result = guard.index_files(index_file_query.scanned_files);

        drop(guard);

        match index_result {
            Ok(v) => {
                let successfully_indexed_files = files_to_index_count - v.len();
                info!("Successfully indexed {} files, {} failed", successfully_indexed_files, v.len());

                return Ok(Response::new(Empty::default()));
            }
            Err(e) => {
                error!("There was an error during indexing. {:?}", e);

                return Err(Status::internal("There was an internal server error."));
            }
        }
    }

    async fn search_file_by_contents(
        self: &Self,
        request: Request<SearchFileByContentsQuery>,
    ) -> Result<Response<SearchFileResponse>, Status> {
        info!("starting search for file");

        let search_file_by_contents_query = request.into_inner();
        let string_query = search_file_by_contents_query.query.to_string();

        let result = self.search_file_strategy.search_file(&string_query);

        return Ok(Response::new(SearchFileResponse { path: result }));
    }

    async fn find_duplicated_files(&self, request: Request<FindDuplicatedFilesQuery>) -> Result<Response<FindDuplicatedFilesResponse>, Status> {
        // TODO: Add pagination
        // let query = request.into_inner();
        // TODO: implement searching for duplicated files starting at a specific path.
        info!("Received request to find duplicated files.");
        let duplicated_files = self.find_duplicated_files_strategy.find_duplicated_files(0);
        info!("Found {} duplicated files.", duplicated_files.len());

        Ok(Response::new(FindDuplicatedFilesResponse { files: duplicated_files }))
    }
}
