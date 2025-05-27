use std::path::Path;

use super::download;
use super::types;


// ---------------------
// Checks if file Exist
// ---------------------
pub fn check_file(
    types::Download {
        directory,
        file_name,
        url,
    }: types::Download,
) {
    // ----------------------
    // check if file does not exist
    // ----------------------
    let full_path = format!("{}/{}", directory, file_name);

    if !Path::new(&full_path).exists() {
        download::new(types::Download {
            url,
            directory,
            file_name,
        });
    }
}
