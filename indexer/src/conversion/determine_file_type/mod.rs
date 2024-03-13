pub(crate) mod determine_file_type_by_extension;
pub(crate) mod determine_file_type_by_human_readability;
pub(crate) mod determine_file_type_lib_magic;

use crate::conversion::convert_to_clear_text_strategy::MimeType;
use crate::conversion::determine_file_type::determine_file_type_by_extension::DetermineFileTypeByExtensionFactory;
use crate::conversion::determine_file_type::determine_file_type_by_human_readability::DetermineFileTypeByHumanReadabilityFactory;
use crate::conversion::determine_file_type::determine_file_type_lib_magic::DetermineFileTypeLibMagic;
use crate::file_indexer::ScannedFile;
use std::sync::Arc;

pub(crate) trait DetermineFileTypeStrategy: Send + Sync {
    fn determine(&self, file: &ScannedFile) -> Option<MimeType>;
}

pub(crate) struct DefaultDetermineFileType {
    /// The given strategies will be gone through in the given order to determine the file type.
    strategies: Vec<Arc<dyn DetermineFileTypeStrategy>>,
}

impl DefaultDetermineFileType {
    pub(crate) fn new(strategies: Vec<Arc<dyn DetermineFileTypeStrategy>>) -> Self {
        return Self { strategies };
    }
}

impl DetermineFileTypeStrategy for DefaultDetermineFileType {
    fn determine(&self, file: &ScannedFile) -> Option<MimeType> {
        for strategy in &self.strategies {
            let res = strategy.determine(file);

            if res.is_some() {
                return res;
            }
        }

        return None;
    }
}

pub(crate) struct DefaultDetermineFileTypeFactory;

impl DefaultDetermineFileTypeFactory {
    pub fn create() -> DefaultDetermineFileType {
        let by_extension_strategy = Arc::new(DetermineFileTypeByExtensionFactory::create());
        let by_human_readability_strategy =
            Arc::new(DetermineFileTypeByHumanReadabilityFactory::create());
        let by_lib_magic_strategy = Arc::new(DetermineFileTypeLibMagic::new());

        let strategies: Vec<Arc<dyn DetermineFileTypeStrategy>> = vec![
            by_extension_strategy,
            by_human_readability_strategy,
            by_lib_magic_strategy,
        ];

        return DefaultDetermineFileType::new(strategies);
    }
}
