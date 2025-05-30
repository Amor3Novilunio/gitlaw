use std::path::Path;

use super::download;
use super::types;

// ---------------------
// Checks if file Exist
// ---------------------
pub fn check_file(
    types::New {
        directory,
        file_name,
        url,
    }: types::New,
) {
    // ----------------------
    // check if file does not exist
    // ----------------------
    let full_path = format!("{}/{}", directory, file_name);

    // ------------------------------------
    // If File Does Not Exist Run Download
    // ------------------------------------
    if !Path::new(&full_path).exists() {
        download::new(types::New {
            url,
            directory,
            file_name,
        });
    }
}
