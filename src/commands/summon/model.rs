use gitlaw::modules::download::{file::check_file, types::New};
use gitlaw::modules::toml::read::config;

// ----------------
// Download Model
// ----------------
pub fn summon_model() {
    // ----------------
    // read url from toml
    // ----------------
    let config = config();

    // ---------
    // Download
    // ---------
    check_file(New {
        url: config.download.model,
        directory: config.model.path,
        file_name: config.model.file_name,
    });
}
