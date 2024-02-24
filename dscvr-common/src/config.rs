use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub indexer: IndexerSettings,
    pub common: CommonSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonSettings {
    pub base_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexerSettings {
    pub host: String,
    pub port: String,
    pub index_directory_name: String,
    pub db_file_name: String,
}

pub fn init_config() -> AppSettings {
    let settings = Config::builder()
        .add_source(config::File::with_name("./default_settings"))
        .add_source(config::Environment::with_prefix("DSCVR"))
        .build()
        .unwrap();

    settings.try_deserialize::<AppSettings>().unwrap()
}
