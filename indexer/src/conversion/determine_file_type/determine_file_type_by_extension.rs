use std::collections::HashMap;
use std::path::Path;
use crate::conversion::convert_to_clear_text_strategy::MimeType;
use crate::conversion::determine_file_type::DetermineFileTypeStrategy;
use crate::file_indexer::ScannedFile;

pub(crate) struct DetermineFileTypeByExtension {
    file_extension_mime_type_map: HashMap<String, MimeType>
}

impl DetermineFileTypeByExtension {
    pub fn new(file_extension_mime_type_map: HashMap<String, MimeType>) -> Self {
        Self {file_extension_mime_type_map}
    }
}

impl DetermineFileTypeStrategy for DetermineFileTypeByExtension {
    fn determine(&self, file: &ScannedFile) -> Option<MimeType> {
        let extension = Path::new(&file.path).extension()?.to_str()?.to_string();

        let mime_type = self.file_extension_mime_type_map.get(&extension)?.clone();

        return Some(mime_type);
    }
}

struct DetermineFileTypeByExtensionFactory;

impl DetermineFileTypeByExtensionFactory {
    pub fn create() -> DetermineFileTypeByExtension {
        return DetermineFileTypeByExtension::new(Self::get_file_type_mime_type_map());
    }

    pub fn get_file_type_mime_type_map() -> HashMap<String, MimeType> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_file_name_ending_with_pdf_returns_pdf() {
        let file = ScannedFile {
            path: "/home/user/documents/important-pdf.pdf".to_string(),
            readable: true,
            hash: "abcdefghijklmnopqrstuvwxyz".to_string(),
        };
        let strategy_under_test = DetermineFileTypeByExtensionFactory::create();

        let result = strategy_under_test.determine(&file);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), MimeType::ApplicationPdf);
    }

    #[test]
    fn test_if_file_name_without_ending_returns_nothing() {
        let file = ScannedFile {
            path: "/home/user/.local/bin/important-script".to_string(),
            readable: true,
            hash: "zyxwvutsrqponmlkjihgfedcba".to_string(),
        };
        let strategy_under_test = DetermineFileTypeByExtensionFactory::create();

        let result = strategy_under_test.determine(&file);

        assert!(result.is_none());
    }
}