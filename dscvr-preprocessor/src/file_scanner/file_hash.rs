use anyhow::Error;
use memmap2::Mmap;
use std::fs::File;
use std::path::PathBuf;

pub(crate) trait FileHashStrategy: Send + Sync {
    fn calculate_hash(&self, path: &PathBuf) -> Result<String, Error>;
}

pub(crate) struct FileHash;

impl FileHash {
    pub fn new() -> Self {
        return FileHash {};
    }
}

impl FileHashStrategy for FileHash {
    fn calculate_hash(&self, path: &PathBuf) -> Result<String, Error> {
        // let file = File::open(path)?;
        // let memory_file = unsafe { Mmap::map(&file)? };
        // let hash = blake3::hash(&memory_file);
        //
        // return Ok(hash.to_string());
        return Ok("".to_string());
    }
}
