use super::create::create_toml;
use super::types::{Config, Mode};
use crate::std_error_exit;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize)]
pub struct AiTable {
    pub mode: Mode,
    pub path: String,
    pub model: String,
    url: String,
    api_key: String,
    pub temperature: f32,
}

pub fn read_from_file() -> Config<AiTable> {
    let path: &str = "./";

    let load_toml_config = match read_to_string("gitlaw.toml") {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to load gitlaw.toml {}", err)),
    };

    if load_toml_config.is_empty() {
        create_toml(path);
        return read_from_file();
    }

    match toml::from_str::<Config<AiTable>>(&load_toml_config) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Invalid TOML format {}", err)),
    }
}
