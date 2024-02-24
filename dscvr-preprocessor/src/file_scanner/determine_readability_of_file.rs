use log::error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub(crate) trait DetermineReadablityOfFileStrategy: Send + Sync {
    fn is_file_readable(&self, path: &PathBuf) -> bool;
}

pub(crate) struct DetermineReadabilityOfFile;

impl DetermineReadabilityOfFile {
    pub fn new() -> Self {
        return DetermineReadabilityOfFile {};
    }
}

impl DetermineReadablityOfFileStrategy for DetermineReadabilityOfFile {
    fn is_file_readable(&self, path: &PathBuf) -> bool {
        let mut file = match File::open(path) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "There was an error while trying to open {:?}: {:?}",
                    path, e
                );
                return false;
            }
        };

        let mut buf = [0; 4096];
        let n = match file.read(&mut buf) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "There was an error while trying to read the file {:?}: {:?}",
                    path, e
                );
                return false;
            }
        };

        let readable_chars = buf[..n]
            .iter()
            .filter(|&&c| c.is_ascii_graphic() || c.is_ascii_whitespace())
            .count();

        return readable_chars as f32 / n as f32 > 0.7;
    }
}
