use super::read_helper::extract_table;
use super::types::{ModelTable, DownloadTable, EngineTable};

// ---------------------------
// Extract Model Table From Toml
// ---------------------------
pub fn extract_model_table() -> ModelTable {
    extract_table::<ModelTable>()
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
