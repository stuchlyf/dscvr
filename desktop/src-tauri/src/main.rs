// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod blocking_client;

use crate::blocking_client::BlockingClient;
use crate::file_indexer_service::{
    DuplicatedFile, FindDuplicatedFilesQuery, SearchFileByContentsQuery,
};
use log::{error, info};
use tonic::Request;

pub mod file_indexer_service {
    tonic::include_proto!("file_indexer");
}
pub mod proto_utils {
    tonic::include_proto!("proto_utils");
}

#[tauri::command(rename_all = "camelCase")]
fn search_for_file(query: &str) -> Vec<String> {
    let mut client = match BlockingClient::connect("http://127.0.0.1:50051") {
        Ok(v) => v,
        Err(e) => {
            error!(
                "There was an error while trying to connect to the service: {:?}",
                e
            );
            return Vec::with_capacity(0);
        }
    };

    let response = match client.search_file_by_contents(Request::new(SearchFileByContentsQuery {
        query: query.to_string(),
    })) {
        Ok(v) => v,
        Err(e) => {
            error!(
                "There was an error while trying to search for the given query: {:?}",
                e
            );
            return Vec::with_capacity(0);
        }
    };

    return response.into_inner().path;
}

#[tauri::command(rename_all = "camelCase")]
fn find_duplicated_files() -> Vec<DuplicatedFile> {
    let mut client = match BlockingClient::connect("http://127.0.0.1:50051") {
        Ok(v) => v,
        Err(e) => {
            error!(
                "There was an error while trying to connect to the service: {:?}",
                e
            );
            return Vec::with_capacity(0);
        }
    };

    let response = match client.find_duplicated_files(Request::new(FindDuplicatedFilesQuery {
        starting_at_path: None,
    })) {
        Ok(v) => {
            info!("Request was successful");
            v
        }
        Err(e) => {
            error!(
                "There was an error while trying to find the duplicated files: {:?}",
                e
            );
            return Vec::with_capacity(0);
        }
    };

    let files = response.into_inner().files;

    info!("Retrieved {} duplicated files", files.len());

    return files;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_for_file,
            find_duplicated_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
