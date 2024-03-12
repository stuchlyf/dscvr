pub(crate) mod determine_file_type_by_extension;
pub(crate) mod determine_file_type_by_human_readability;
pub(crate) mod determine_file_type_lib_magic;

use std::collections::HashMap;
use std::sync::Arc;
use crate::conversion::convert_to_clear_text_strategy::MimeType;
use crate::conversion::determine_file_type::determine_file_type_by_extension::DetermineFileTypeByExtension;
use crate::conversion::determine_file_type::determine_file_type_by_human_readability::DetermineFileTypeByHumanReadability;
use crate::conversion::determine_file_type::determine_file_type_lib_magic::DetermineFileTypeLibMagic;
use crate::file_indexer::ScannedFile;

pub(crate) trait DetermineFileTypeStrategy: Send + Sync {
    fn determine(&self, file: &ScannedFile) -> Option<MimeType>;
}

pub(crate) struct DefaultDetermineFileType {
    /// The given strategies will be gone through in the given order to determine the file type.
    strategies: Vec<Arc<dyn DetermineFileTypeStrategy>>
}

impl DefaultDetermineFileType {
    pub(crate) fn new(strategies: Vec<Arc<dyn DetermineFileTypeStrategy>>) -> Self {
        return Self {strategies};
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
        let by_extension_strategy = Arc::new(DetermineFileTypeByExtension::new(Self::get_file_type_mime_type_map()));
        let by_human_readability_strategy = Arc::new(DetermineFileTypeByHumanReadability::new());
        let by_lib_magic_strategy = Arc::new(DetermineFileTypeLibMagic::new());

        let strategies: Vec<Arc<dyn DetermineFileTypeStrategy>> = vec![by_extension_strategy, by_human_readability_strategy, by_lib_magic_strategy];

        return DefaultDetermineFileType::new(strategies);
    }

    fn get_file_type_mime_type_map() -> HashMap<String, MimeType> {
        let mut map = HashMap::new();
        // TODO: Find a better way to add even more types
        map.insert("txt".to_owned(), MimeType::TextPlain);
        map.insert("xml".to_owned(), MimeType::TextPlain);
        map.insert("json".to_owned(), MimeType::TextPlain);
        map.insert("md".to_owned(), MimeType::TextPlain);
        map.insert("ascii".to_owned(), MimeType::TextPlain);
        map.insert("env".to_owned(), MimeType::TextPlain);
        map.insert("gitignore".to_owned(), MimeType::TextPlain);
        map.insert("taurignore".to_owned(), MimeType::TextPlain);
        map.insert("lock".to_owned(), MimeType::TextPlain);
        map.insert("toml".to_owned(), MimeType::TextPlain);
        map.insert("yaml".to_owned(), MimeType::TextPlain);
        map.insert("rs".to_owned(), MimeType::TextPlain);
        map.insert("js".to_owned(), MimeType::TextPlain);
        map.insert("ts".to_owned(), MimeType::TextPlain);
        map.insert("c".to_owned(), MimeType::TextPlain);
        map.insert("cpp".to_owned(), MimeType::TextPlain);
        map.insert("cs".to_owned(), MimeType::TextPlain);
        map.insert("php".to_owned(), MimeType::TextPlain);
        map.insert("html".to_owned(), MimeType::TextPlain);
        map.insert("css".to_owned(), MimeType::TextPlain);
        map.insert("csv".to_owned(), MimeType::TextPlain);
        map.insert("iml".to_owned(), MimeType::TextPlain);
        map.insert("cmd".to_owned(), MimeType::TextPlain);
        map.insert("ps1".to_owned(), MimeType::TextPlain);
        map.insert("sh".to_owned(), MimeType::TextPlain);
        map.insert("bash".to_owned(), MimeType::TextPlain);
        map.insert("fish".to_owned(), MimeType::TextPlain);
        map.insert("java".to_owned(), MimeType::TextPlain);
        map.insert("bat".to_owned(), MimeType::TextPlain);
        map.insert("py".to_owned(), MimeType::TextPlain);
        map.insert("tsx".to_owned(), MimeType::TextPlain);
        map.insert("jsx".to_owned(), MimeType::TextPlain);
        map.insert("gradle".to_owned(), MimeType::TextPlain);
        map.insert("properties".to_owned(), MimeType::TextPlain);
        map.insert("groovy".to_owned(), MimeType::TextPlain);

        map.insert("avif".to_string(), MimeType::ImageAvif);

        map.insert("bmp".to_string(), MimeType::ImageBmp);

        map.insert("doc".to_string(), MimeType::ApplicationMsWord);

        map.insert("docx".to_string(), MimeType::ApplicationVndOpenxmlformatsOfficedocumentWordprocessingmlDocument);

        map.insert("gif".to_string(), MimeType::ImageGif);

        map.insert("ico".to_string(), MimeType::ImageIcon);

        map.insert("jpg".to_string(), MimeType::ImageJpeg);
        map.insert("jpeg".to_string(), MimeType::ImageJpeg);

        map.insert("odp".to_string(), MimeType::ApplicationVndOasisOpendocumentPresentation);

        map.insert("ods".to_string(), MimeType::ApplicationVndOasisOpendocumentSpreadsheet);

        map.insert("odt".to_string(), MimeType::ApplicationVndOasisOpendocumentText);

        map.insert("png".to_string(), MimeType::ImagePng);

        map.insert("pdf".to_string(), MimeType::ApplicationPdf);

        map.insert("ppt".to_string(), MimeType::ApplicationVndMsPowerpoint);

        map.insert("pptx".to_string(), MimeType::ApplicationVndOpenxmlformatsOfficedocumentPresentationmlPresentation);

        map.insert("rtf".to_string(), MimeType::ApplicationRtf);

        map.insert("svg".to_string(), MimeType::ImageSvgXml);

        map.insert("tif".to_string(), MimeType::ImageTiff);
        map.insert("tiff".to_string(), MimeType::ImageTiff);

        map.insert("xls".to_string(), MimeType::ApplicationVndMsExcel);

        map.insert("xlsx".to_string(), MimeType::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet);

        return map;
    }
}