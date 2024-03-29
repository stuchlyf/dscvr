pub(crate) mod clear_text_conversion;
pub(crate) mod convert_to_clear_text_strategy;
pub(crate) mod determine_file_type;
pub(crate) mod pdf_conversion;

use anyhow::Error;

pub(crate) trait Conversion: Send + Sync {
    fn convert(&self, buf: Vec<u8>) -> Result<String, Error>;
}
