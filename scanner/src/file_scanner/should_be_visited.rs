use log::error;
use std::path::PathBuf;

pub(crate) trait ShouldBeVisitedStrategy: Send + Sync {
    fn should_be_visited(&self, path: &PathBuf) -> bool;
}

pub(crate) struct ShouldBeVisited {
    disallowed_entry_names: Vec<String>,
}

impl ShouldBeVisited {
    fn new(disallowed_entry_names: Vec<String>) -> Self {
        return ShouldBeVisited {
            disallowed_entry_names,
        };
    }
}

impl Default for ShouldBeVisited {
    fn default() -> Self {
        return ShouldBeVisited::new(vec![
            String::from("node_modules"),
            String::from("WpSystem"),
            String::from(".cache"),
            String::from("cache"),
        ]);
    }
}

impl ShouldBeVisitedStrategy for ShouldBeVisited {
    fn should_be_visited(&self, path: &PathBuf) -> bool {
        let file_name = match path.file_name() {
            Some(v) => v,
            None => {
                error!(
                    "There was an error while trying to get the file name of given path {:?}.",
                    path
                );
                return false;
            }
        };

        let file_name = match file_name.to_str() {
            Some(v) => v,
            None => {
                error!(
                    "There was an error while trying to convert the file_name ({:?}) to a '&str'.",
                    file_name
                );
                return false;
            }
        };

        return !self.disallowed_entry_names.contains(&file_name.to_string());
    }
}
