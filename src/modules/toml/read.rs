use super::read_tables::{extract_model_table, extract_download_table, extract_engine_table};
use super::types::{ModelColumns, Config, DownloadColumns, EngineColumns};

// -----------------------------------
// Get All Toml Table Data
// works like a hook(React Reference)
// -----------------------------------
pub fn config() -> Config<ModelColumns, EngineColumns, DownloadColumns> {
    Config {
        model: extract_model_table().model,
        engine: extract_engine_table().engine,
        download: extract_download_table().download,
    }
}
