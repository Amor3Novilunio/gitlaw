use serde::Serialize;
use std::fs::write;

use crate::std_error_exit;

#[derive(Serialize)]
enum Mode {
    Online,
    Offline,
}

#[derive(Serialize)]
pub struct AiTable {
    mode: Mode,
    path: String,
    model: String,
    url: String,
    api_key: String,
    temperature: f32,
}

#[derive(Serialize)]
pub struct Config {
    pub ai: AiTable,
}

pub fn create_toml(config: &Config, path: &str) {
    let parsed_config = match toml::to_string_pretty(config) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("{}", err)),
    };

    match write(path, parsed_config) {
        Ok(res)=>res,
        Err(err) => std_error_exit!(format!("{}", err)),
    }
}
