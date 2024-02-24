use anyhow::anyhow;
use chrono::{DateTime, Local};
use dscvr_common::config::AppSettings;
use log::error;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub(crate) struct FileMetadata {
    pub path: String,
    pub size: u64,
    pub indexed_at: DateTime<Local>,
    pub hash: String,
}

pub(crate) trait PersistMetadataStrategy: Send + Sync {
    fn persist_metadata(&self, metadata: FileMetadata) -> Result<(), anyhow::Error>;
}

pub(crate) struct SqlitePersistenceStrategy {
    conn: Arc<Mutex<Connection>>,
}

impl SqlitePersistenceStrategy {
    pub(crate) fn build_with_settings(settings: &AppSettings) -> Result<Self, anyhow::Error> {
        let path_to_db = Path::new(&settings.common.base_dir).join(&settings.indexer.db_file_name);

        let connection = Connection::open(path_to_db)?;

        let instance = SqlitePersistenceStrategy {
            conn: Arc::new(Mutex::new(connection)),
        };

        Self::initialize_db(&instance)?;

        return Ok(instance);
    }

    fn initialize_db(&self) -> Result<(), anyhow::Error> {
        let guard = match self.conn.lock() {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "There was an error while trying to acquire the lock for the connection: {}",
                    e
                );
            }
        };

        guard.execute(
            "CREATE TABLE IF NOT EXISTS indexed_files (path VARCHAR(256) PRIMARY KEY, hash VARCHAR(64) NOT NULL, size INT(64) NOT NULL, indexed_at DATETIME NOT NULL)",
            ()
        )?;

        drop(guard);

        return Ok(());
    }
}

impl PersistMetadataStrategy for SqlitePersistenceStrategy {
    fn persist_metadata(&self, metadata: FileMetadata) -> Result<(), anyhow::Error> {
        let guard = match self.conn.lock() {
            Ok(v) => v,
            Err(e) => {
                error!("There was an error while trying to acquire the lock for the connection when trying to insert metadata: {:?}", e);
                return Err(anyhow!("Poison Error"));
            }
        };

        guard.execute(
            "\
                INSERT INTO indexed_files (path, hash, size, indexed_at) VALUES (?, ?, ?, ?);
            ",
            (
                &metadata.path,
                &metadata.hash,
                &metadata.size,
                &metadata.indexed_at,
            ),
        )?;

        drop(guard);

        return Ok(());
    }
}
