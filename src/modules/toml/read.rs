use super::read_tables::{extract_ai_table, extract_download_table, extract_engine_table};
use super::types::{AiColumns, Config, DownloadColumns, EngineColumns};

// ----------------------
// Get All Toml Table Data
// works like a hook(React Reference)
// ----------------------
pub fn read_from_file() -> Config<AiColumns, EngineColumns, DownloadColumns> {
    Config {
        ai: extract_ai_table().ai,
        engine: extract_engine_table().engine,
        download: extract_download_table().download,
    }
}
