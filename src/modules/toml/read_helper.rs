use std::fs::read_to_string;

use crate::std_error_exit;
use serde::de::DeserializeOwned;

use toml::from_str;

use super::create::create_toml;

// ----------------------
// Extract table Configuration
// ----------------------
pub fn extract_table<T: DeserializeOwned>() -> T {
    // ----------------------
    // Toml Name & Path Location
    // ----------------------
    let path: &str = "./gitlaw.toml";

    // ----------------------
    // Load Toml Configuration
    // ----------------------
    let load_toml_config = match read_to_string("gitlaw.toml") {
        Ok(res) => res,
        Err(_) => String::new(),
    };

    // ----------------------
    // Toml Error Handling
    // ----------------------
    if load_toml_config.is_empty() {
        create_toml(path);
        return extract_table();
    }

    // ----------------------
    // Deserialized Toml Data
    // ----------------------
    match from_str::<T>(&load_toml_config) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Invalid TOML format | {}", err)),
    }
}
