use crate::conversion::Conversion;
use anyhow::Error;

pub(crate) struct ClearTextConversion;

impl ClearTextConversion {
    pub(crate) fn new() -> Self {
        return Self {};
    }
}

impl Conversion for ClearTextConversion {
    fn convert(&self, buf: Vec<u8>) -> Result<String, Error> {
        let contents = String::from_utf8(buf)?; // TODO: Handle non UTF-8 Files

        return Ok(contents);
    }
}
