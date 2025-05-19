use serde::Deserialize;
use std::fs::read_to_string;

use crate::std_error_exit;

#[derive(Deserialize)]
enum Mode {
    Online,
    Offline,
}

#[derive(Deserialize)]
pub struct GitlawToml {
    pub mode: Mode,
    pub path: String,
    pub model: String,
    url: String,
    api_key: String,
    pub temperature: f32
}

#[derive(Deserialize)]
pub struct TomlConfig {
    pub ai: GitlawToml,
}

pub fn load_from_file() -> TomlConfig {
    let load_toml_config = match read_to_string("gitlaw.toml") {
        Ok(result) => result,
        Err(err) => std_error_exit!(format!("Failed to load gitlaw.toml {}", err)),
    };

    match toml::from_str::<TomlConfig>(&load_toml_config) {
        Ok(parsed_result) => parsed_result,
        Err(err) => std_error_exit!(format!("Invalid TOML format {}", err)),
    }
}
