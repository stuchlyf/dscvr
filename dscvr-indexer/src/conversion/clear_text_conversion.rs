use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use anyhow::Error;
use crate::conversion::Conversion;

pub(crate) struct ClearTextConversion;

impl ClearTextConversion {
    pub(crate) fn new() -> Self {
        return Self {};
    }
}

impl Conversion for ClearTextConversion {
    fn convert(&self, path: &PathBuf) -> Result<String, Error> {
        let mut file = File::open(&path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        return Ok(contents);
    }
}

