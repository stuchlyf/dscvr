use crate::conversion::convert_to_clear_text_strategy::MimeType;
use crate::conversion::determine_file_type::DetermineFileTypeStrategy;
use crate::file_indexer::ScannedFile;

pub(crate) struct DetermineFileTypeLibMagic;

impl DetermineFileTypeLibMagic {
    pub fn new() -> Self {
        return Self {};
    }
}

impl DetermineFileTypeStrategy for DetermineFileTypeLibMagic {
    fn determine(&self, _file: &ScannedFile) -> Option<MimeType> {
        // todo!()
        return None;
    }
}