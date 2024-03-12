use crate::config::AppSettings;

pub mod config;
pub mod logger;
pub mod utils;

pub trait BuildableFromSettings {
    fn build_from_settings(settings: &AppSettings) -> Self;
}
