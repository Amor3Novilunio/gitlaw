use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Mode {
    Online,
    Offline,
}

#[derive(Deserialize, Serialize)]
pub struct Config<T> {
    pub ai: T,
}
