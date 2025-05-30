use gitlaw::modules::download::{file::check_file, types::New};
use gitlaw::modules::toml::read::config;

// ----------------
// Download Engine
// ----------------
pub fn summon_engine() {
    // ----------------
    // read url from toml
    // ----------------
    let config = config();

    // ---------
    // Download
    // ---------
    check_file(New {
        url: config.download.engine,
        directory: config.engine.path,
        file_name: config.engine.file_name,
    });
}
