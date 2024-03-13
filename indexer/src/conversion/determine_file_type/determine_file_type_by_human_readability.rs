use std::fs::File;
use std::path::Path;
use memmap2::Mmap;
use crate::conversion::convert_to_clear_text_strategy::MimeType;
use crate::conversion::determine_file_type::DetermineFileTypeStrategy;
use crate::file_indexer::ScannedFile;

pub(crate) struct DetermineFileTypeByHumanReadability;

impl DetermineFileTypeByHumanReadability {
    pub fn new() -> Self {
        return Self {};
    }
}

impl DetermineFileTypeStrategy for DetermineFileTypeByHumanReadability {
    fn determine(&self, file: &ScannedFile) -> Option<MimeType> {
        let path_to_file = Path::new(&file.path);
        let file = File::open(path_to_file).ok()?; // TODO: logging

        let memory_file = unsafe { Mmap::map(&file).ok()? }; // TODO: logging

        let readable_chars = memory_file
            .iter()
            .filter(|&&c| c.is_ascii_graphic() || c.is_ascii_whitespace())
            .count();

        if readable_chars as f32 / memory_file.len() as f32 > 0.7 {
            return Some(MimeType::TextPlain);
        } else {
            return None;
        }
    }
}

pub(crate) struct DetermineFileTypeByHumanReadabilityFactory;

impl DetermineFileTypeByHumanReadabilityFactory {
    pub fn create() -> DetermineFileTypeByHumanReadability {
        return DetermineFileTypeByHumanReadability::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;
    use std::path::{Path, PathBuf};
    use crate::file_indexer::ScannedFile;

    #[test]
    fn test_if_text_file_is_determined_as_text_plain_file() {
        let test_data_directory = get_test_data_directory();
        let test_file_path_buf = test_data_directory
            .join("01")
            .join("this_is_an_important_file_for_testing.txt");

        let test_file_path_as_str = test_file_path_buf.to_str().unwrap();

        let file = ScannedFile {
            path: test_file_path_as_str.to_string()
        };

        let under_test = DetermineFileTypeByHumanReadabilityFactory::create();

        let result = under_test.determine(&file);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), MimeType::TextPlain);
    }

    #[test]
    fn test_if_non_text_file_is_determined_as_unknown_file() {
        let test_data_directory = get_test_data_directory();
        let test_file_path_buf = test_data_directory
            .join("01")
            .join("this_is_a_random_google_docs_file.docx");

        let test_file_path_as_str = test_file_path_buf.to_str().unwrap();

        let file = ScannedFile {
            path: test_file_path_as_str.to_string()
        };

        let under_test = DetermineFileTypeByHumanReadabilityFactory::create();

        let result = under_test.determine(&file);

        assert!(result.is_none());
    }


    fn get_test_data_directory() -> PathBuf {
        let current_dir_path_buf = current_dir().unwrap();

        let current_path = Path::new(current_dir_path_buf.to_str().unwrap());
        let mut ancestor_iter = current_path.ancestors();
        ancestor_iter.next();
        let base_path = ancestor_iter.next().unwrap();

        return base_path.to_path_buf();
    }
}