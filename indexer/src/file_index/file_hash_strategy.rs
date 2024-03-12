use anyhow::Error;

pub(crate) trait FileHashStrategy: Send + Sync {
    fn calculate_hash(&self, buf: &Vec<u8>) -> Result<String, Error>;
}

pub(crate) struct DefaultFileHash;

impl DefaultFileHash {
    pub fn new() -> Self {
        return DefaultFileHash {};
    }
}

impl FileHashStrategy for DefaultFileHash {
    fn calculate_hash(&self, buf: &Vec<u8>) -> Result<String, Error> {
        let hash = blake3::hash(&buf);

        return Ok(hash.to_string());
    }
}
