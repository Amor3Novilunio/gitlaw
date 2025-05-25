use serde::{Deserialize, Serialize};

// ----------------------
// Table Config
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct Config<Ai, Engine, Download> {
    pub ai: Ai,
    pub engine: Engine,
    pub download: Download,
}

// ----------------------
// Ai Table
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct AiTable {
    pub ai: AiColumns,
}

// ----------------------
// Ai Table Columns
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct AiColumns {
    pub path: String,
    pub temperature: f32,
}

// ----------------------
// Engine Table
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct EngineTable {
    pub engine: EngineColumns,
}

// ----------------------
// Engine Columns
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct EngineColumns {
    pub path: String,
}

// ----------------------
// Download Table
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct DownloadTable {
    pub download: DownloadColumns,
}

// ----------------------
// Download Columns
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct DownloadColumns {
    pub engine: String,
    pub model: String,
}
