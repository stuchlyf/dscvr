use crate::{DATABASE_NAME, DSCVR_BASE_DIR_ENVIRONMENT_VARIABLE_NAME, FALLBACK_DSCVR_BASE_DIR};
use anyhow::Error;
use rusqlite::Connection;
use std::env;
use std::fs::create_dir;
use std::path::Path;

pub(crate) fn connect_to_db() -> Result<Connection, Error> {
    let base_dir = env::var(DSCVR_BASE_DIR_ENVIRONMENT_VARIABLE_NAME)
        .unwrap_or_else(|_| FALLBACK_DSCVR_BASE_DIR.to_string());

    let base_path = Path::new(&base_dir);

    if !base_path.exists() {
        create_dir(base_path)?
    }

    let full_path = base_path.join(DATABASE_NAME);

    match Connection::open(full_path) {
        Ok(v) => return Ok(v),
        Err(e) => {
            return Err(Error::from(e));
        }
    };
}

pub(crate) fn init_db(conn: &Connection) -> Result<(), Error> {
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS scanned_files (path VARCHAR(256) PRIMARY KEY, hash VARCHAR(64) NOT NULL, size INT(64) NOT NULL, scanned_at DATETIME NOT NULL)",
        ()
    )?;

    return Ok(());
}
