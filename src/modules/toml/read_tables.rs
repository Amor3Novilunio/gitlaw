use super::read_helper::extract_table;
use super::types::{Config, Mode};
use serde::Deserialize;

// ----------------------
// Ai Table Model
// ----------------------
#[derive(Deserialize)]
pub struct AiTable {
    pub mode: Mode,
    pub path: String,
    pub model: String,
    url: String,
    api_key: String,
    pub temperature: f32,
}

// ----------------------
// Extract ai Table From Toml
// ----------------------
pub fn extract_ai_table() -> Config<AiTable> {
    extract_table::<AiTable>()
}
