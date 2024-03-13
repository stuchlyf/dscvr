use crate::conversion::Conversion;
use anyhow::{anyhow, Error};
use std::panic;

pub(crate) struct PdfConversion;

impl PdfConversion {
    pub(crate) fn new() -> Self {
        return Self {};
    }
}

impl Conversion for PdfConversion {
    fn convert(&self, buf: Vec<u8>) -> Result<String, Error> {
        let out = match panic::catch_unwind(|| pdf_extract::extract_text_from_mem(&buf)) {
            Ok(v) => v?,
            Err(e) => {
                return Err(anyhow!(
                    "There was an unknown error while trying to extract text from a pdf. {:?}",
                    e
                ))
            }
        };

        return Ok(out);
    }
}
