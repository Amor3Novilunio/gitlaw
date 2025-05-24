use super::types::{Config, Mode};
use crate::std_error_exit;
use serde::Serialize;
use std::fs::write;

// ----------------------
// Ai Table Model
// ----------------------
#[derive(Serialize)]
pub struct AiTable {
    mode: Mode,
    path: String,
    model: String,
    url: String,
    api_key: String,
    temperature: f32,
}

pub fn create_toml(path: &str) {
    // ----------------------
    // Default Configuration
    // ----------------------
    let config = Config {
        ai: AiTable {
            mode: Mode::Offline,
            path: "".into(),
            model: "".into(),
            url: "".into(),
            api_key: "".into(),
            temperature: 0.7,
        },
    };

    // ----------------------
    // Serialize Config
    // ----------------------
    let parsed_config = match toml::to_string_pretty(&config) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("failed To Serialize Toml Config | {}", err)),
    };

    // ----------------------
    // Create Config File
    // ----------------------
    match write(path, parsed_config) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("failed to Create Config File | {}", err)),
    }
}
