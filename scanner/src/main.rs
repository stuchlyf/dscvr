extern crate dotenv;
#[macro_use]
extern crate derive_builder;

use crate::db::{connect_to_db, init_db};
use crate::file_indexer_service::file_indexer_client::FileIndexerClient;
use crate::file_indexer_service::IndexFileQuery;
use crate::file_scanner::determine_readability_of_file::DetermineReadabilityOfFile;
use crate::file_scanner::file_hash::FileHash;
use crate::file_scanner::should_be_visited::ShouldBeVisited;
use crate::file_scanner::{FileScanner, Scanner};
use dotenv::dotenv;
use dscvr_common::config::init_config;
use dscvr_common::logger::init_logger;
use log::info;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::{env, thread};
use tonic::Request;

mod db;
mod file_scanner;
mod utils;

pub mod file_indexer_service {
    tonic::include_proto!("file_indexer");
}
pub mod proto_utils {
    tonic::include_proto!("proto_utils");
}

pub const DSCVR_BASE_DIR_ENVIRONMENT_VARIABLE_NAME: &str = "DSCVR_HOME";
pub const FALLBACK_DSCVR_BASE_DIR: &str = concat!(env!("APPDATA"), "/dscvr");
pub const DATABASE_NAME: &str = "scanned_files.sqlite";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let settings = init_config();
    init_logger();
    let conn = connect_to_db().expect("To be able to create the connection to the database");
    init_db(&conn)?;

    let mut client = FileIndexerClient::connect(format!(
        "http://{}:{}",
        settings.indexer.host, settings.indexer.port
    ))
    .await?;

    let path_to_directory = env::var("DSCVR_TEST_DIR")?;

    let should_be_visited_strategy = Arc::new(ShouldBeVisited::default());
    let determine_readability_of_file = Arc::new(DetermineReadabilityOfFile::new());
    let file_hash_strategy = Arc::new(FileHash::new());

    let file_scanner = Scanner::builder()
        .should_be_visited_strategy(should_be_visited_strategy)
        .determine_readability_of_file(determine_readability_of_file)
        .file_hash_strategy(file_hash_strategy)
        .build()?;

    let scanned_files = file_scanner.scan_directory(Path::new(&path_to_directory))?;

    // TODO: Handle Errors
    let successfully_scanned_files = scanned_files
        .into_iter()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    info!(
        "There are {} successfully scanned files.",
        successfully_scanned_files.len()
    );

    let chunked_scanned_files = successfully_scanned_files
        .chunks(1_000)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    for chunk in chunked_scanned_files {
        client
            .index_file(Request::new(IndexFileQuery {
                scanned_files: chunk,
            }))
            .await?;
    }

    return Ok(());
}
