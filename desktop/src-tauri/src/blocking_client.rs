use crate::file_indexer_service::file_indexer_client::FileIndexerClient;
use crate::file_indexer_service::{
    FindDuplicatedFilesQuery, FindDuplicatedFilesResponse, SearchFileByContentsQuery,
    SearchFileResponse,
};
use tokio::runtime::{Builder, Runtime};
use tonic::transport::Channel;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;

pub(crate) struct BlockingClient {
    client: FileIndexerClient<Channel>,
    rt: Runtime,
}

impl BlockingClient {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let client = rt.block_on(FileIndexerClient::connect(dst))?;

        Ok(Self { client, rt })
    }

    pub fn search_file_by_contents(
        &mut self,
        request: impl tonic::IntoRequest<SearchFileByContentsQuery>,
    ) -> Result<tonic::Response<SearchFileResponse>, tonic::Status> {
        self.rt
            .block_on(self.client.search_file_by_contents(request))
    }

    pub fn find_duplicated_files(
        &mut self,
        request: impl tonic::IntoRequest<FindDuplicatedFilesQuery>,
    ) -> Result<tonic::Response<FindDuplicatedFilesResponse>, tonic::Status> {
        return self.rt.block_on(self.client.find_duplicated_files(request));
    }
}
