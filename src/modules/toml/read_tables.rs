use super::read_helper::extract_table;
use super::types::{AiTable, DownloadTable, EngineTable};

// ---------------------------
// Extract AI Table From Toml
// ---------------------------
pub fn extract_ai_table() -> AiTable {
    extract_table::<AiTable>()
}

// -------------------------------
// Extract engine Table From Toml
// -------------------------------
pub fn extract_engine_table() -> EngineTable {
    extract_table::<EngineTable>()
}

// ---------------------------------
// Extract download Table From Toml
// ---------------------------------
pub fn extract_download_table() -> DownloadTable {
    extract_table::<DownloadTable>()
}
