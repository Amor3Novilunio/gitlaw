use serde::Deserialize;
use std::fs::read_to_string;
use crate::std_error_exit;

#[derive(Deserialize)]
enum Mode {
    Online,
    Offline,
}
#[derive(Deserialize)]
pub struct AiTable {
    pub mode: Mode,
    pub path: String,
    pub model: String,
    url: String,
    api_key: String,
    pub temperature: f32
}

#[derive(Deserialize)]
pub struct Config {
    pub ai: AiTable,
}

pub fn read_from_file() -> Config {
    let load_toml_config = match read_to_string("gitlaw.toml") {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to load gitlaw.toml {}", err)),
    };

    match toml::from_str::<Config>(&load_toml_config) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Invalid TOML format {}", err)),
    }
}