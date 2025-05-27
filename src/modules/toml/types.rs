use serde::{Deserialize, Serialize};

// ----------------------
// Table Config
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct Config<Model, Engine, Download> {
    pub model: Model,
    pub engine: Engine,
    pub download: Download,
}

// ----------------------
// Ai Table
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct ModelTable {
    pub model: ModelColumns,
}

// ----------------------
// Ai Table Columns
// ----------------------
#[derive(Deserialize, Serialize)]
pub struct ModelColumns {
    pub path: String,
    pub file_name: String,
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
    pub file_name: String,
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
