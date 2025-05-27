use super::types::{Config, DownloadColumns, EngineColumns, ModelColumns};
use crate::std_error_exit;
use std::fs::write;

// ------------
// Create Toml
// ------------
pub fn create_toml(path: &str) {
    // ----------------------
    // Default Configuration
    // ----------------------
    let config = Config {
        model: ModelColumns {
            path: "".into(),
            file_name: "".into(),
            temperature: 0.7,
        },
        engine: EngineColumns {
            path: "".into(),
            file_name: "".into(),
        },
        download: DownloadColumns {
            engine: "".into(),
            model: "".into(),
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
