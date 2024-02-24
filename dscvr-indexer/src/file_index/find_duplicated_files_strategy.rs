use std::path::Path;
use std::sync::{Arc, Mutex};
use log::error;
use rusqlite::{Connection};
use dscvr_common::config::AppSettings;
use crate::file_indexer::{DuplicatedFile};

pub(crate) trait FindDuplicatedFilesStrategy: Send + Sync {
    fn find_duplicated_files(&self, page: usize) -> Vec<DuplicatedFile>;
    fn find_duplicated_files_starting_at_path(&mut self, path: &str) -> Vec<DuplicatedFile>;
}

pub(crate) struct FindDuplicatedFiles {
    conn: Arc<Mutex<Connection>>
}

impl FindDuplicatedFiles {
    pub(crate) fn build_with_settings(settings: &AppSettings) -> Result<Self, anyhow::Error> {
        let path_to_db = Path::new(&settings.common.base_dir).join(&settings.indexer.db_file_name);

        let connection = Connection::open(path_to_db)?;

        let instance = FindDuplicatedFiles {
            conn: Arc::new(Mutex::new(connection)),
        };

        return Ok(instance);
    }
}

impl FindDuplicatedFilesStrategy for FindDuplicatedFiles {

    fn find_duplicated_files(&self, page: usize) -> Vec<DuplicatedFile> {
        let offset = page * 500;

        let guard = match self.conn.lock() {
            Ok(v) => v,
            Err(e) => {
                error!("There was an error while trying to acquire the lock for the connection: {:?}", e);
                return Vec::with_capacity(0);
            }
        };

        let mut statement = match guard.prepare("SELECT
                    GROUP_CONCAT(indexed_files.path, ', ') AS 'Paths',
                    COUNT(indexed_files.hash) AS 'Duplicates',
                    SUM(indexed_files.size) AS 'Aggregated Size in Mebibyte',
                    indexed_files.hash
                FROM indexed_files
                GROUP BY hash
                HAVING COUNT(hash) > 1
                ORDER BY SUM(indexed_files.size) DESC
                LIMIT 500
                OFFSET ?") {
            Ok(v) => v,
            Err(e) => {
                error!("There was an error while trying to prepare statement: {:?}", e);
                return Vec::with_capacity(0);
            }
        };

        let rows = match statement.query_map([offset], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }) {
            Ok(v) => {
                v
                    .filter_map(|row| row.ok()) // TODO: handle errors
                    .collect::<Vec<(String, u64, u64, String)>>()
            },
            Err(e) => {
                error!("There was an error while trying to execute the query: {:?}", e);
                return Vec::with_capacity(0);
            }
        };

        return rows
            .into_iter()
            .map(|row| {
                let (paths, duplicates, aggregated_size, hash) = row;
                let paths = paths.split(", ")
                    .into_iter()
                    .map(|path| path.into())
                    .collect::<Vec<String>>();

                DuplicatedFile {
                    paths,
                    aggregated_size,
                    duplicates,
                    hash
                }
            })
            .collect::<Vec<_>>();
    }

    fn find_duplicated_files_starting_at_path(&mut self, path: &str) -> Vec<DuplicatedFile> {
        todo!()
    }
}