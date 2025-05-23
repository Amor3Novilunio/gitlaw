use super::types::{Config};
use super::read_tables::{extract_ai_table,AiTable};

// ----------------------
// Get All Toml Table Data
// works like a hook(React Reference)
// if more than one apply ( config<> ) & { to exctracted value }
// ----------------------
pub fn read_from_file() -> Config<AiTable> {
    extract_ai_table()
}
