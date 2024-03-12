use crate::file_indexer_service::ScannedFile;
use crate::file_scanner::should_be_visited::ShouldBeVisitedStrategy;
use crate::utils::{convert_path_buf_to_string, split_vec_into_chunks};
use anyhow::{anyhow, Error};
use log::{debug, error, info};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) mod should_be_visited;

pub(crate) trait FileScanner {
    fn scan_directory(&self, path: &Path) -> Result<Vec<Result<ScannedFile, Error>>, Error>;
    fn scan_file(&self, path: &Path) -> Result<ScannedFile, Error>;
}

#[derive(Builder)]
pub(crate) struct Scanner {
    should_be_visited_strategy: Arc<dyn ShouldBeVisitedStrategy>,
}

impl Scanner {
    pub(crate) fn builder() -> ScannerBuilder {
        return ScannerBuilder::default();
    }

    fn get_scannable_files_in_directory(&self, path: &Path) -> Result<Vec<PathBuf>, Error> {
        if !path.is_dir() {
            return Err(anyhow!("The given path is not a directory."));
        }

        let scannable_paths = path
            .read_dir()?
            .filter_map(|dir_entry| dir_entry.ok()) // TODO: improve Error handling
            .map(|dir_entry| dir_entry.path())
            .filter(|path| self.should_be_visited_strategy.should_be_visited(path))
            .map(|path_in_dir| {
                if path_in_dir.is_dir() {
                    return self
                        .get_scannable_files_in_directory(&path_in_dir)
                        .unwrap_or_else(|_| vec![]);
                } else {
                    return vec![path_in_dir];
                }
            })
            .flatten()
            .collect();

        return Ok(scannable_paths);
    }
}

impl FileScanner for Scanner {
    fn scan_directory(&self, path: &Path) -> Result<Vec<Result<ScannedFile, Error>>, Error> {
        info!("Starting to scan for paths starting at {:?}", path);
        let paths_to_scan = self.get_scannable_files_in_directory(path)?;
        info!("Found {} paths to scan", paths_to_scan.len());

        let chunks = split_vec_into_chunks(&paths_to_scan);
        let scanner_results = Arc::new(Mutex::new(Vec::with_capacity(paths_to_scan.len())));
        thread::scope(|s| {
            for (chunk_index, chunk) in chunks.iter().enumerate() {
                let scanner_results = Arc::clone(&scanner_results);

                let _ = dscvr_common::utils::spawn_scoped_thread_with_name(
                    s,
                    format!("FileScanner-{}", chunk_index),
                    move || {
                        debug!("Spawned thread {}", chunk_index);

                        for (i, path) in chunk.iter().enumerate() {
                            let scanned_file_result = self.scan_file(path);

                            let mut guard = match scanner_results.lock() {
                                Ok(v) => v,
                                Err(e) => {
                                    error!("There was an error while trying to acquire the lock for the 'scanner_results'-Mutex: {:?}", e);
                                    return;
                                }
                            };

                            guard.push(scanned_file_result);

                            drop(guard);
                        }
                    },
                );
            }
        });

        let scanner_results = Arc::into_inner(scanner_results)
            .unwrap()
            .into_inner()
            .unwrap();

        return Ok(scanner_results);
    }

    fn scan_file(&self, path: &Path) -> Result<ScannedFile, Error> {
        let path = PathBuf::from(path);

        return Ok(ScannedFile {
            path: convert_path_buf_to_string(&path),
        });
    }
}
