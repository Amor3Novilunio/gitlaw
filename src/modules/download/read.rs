use std::{fs::create_dir_all, path::Path};

use super::download::download;
use super::types::Download;

use crate::std_error_exit;

// ---------------------
// Checks if file Exist
// ---------------------
pub fn check_file(
    Download {
        directory,
        file_name,
        url,
    }: Download,
) {
    // ----------------------
    // check if file does not exist
    // ----------------------
    let full_path = format!("{}/{}", directory, file_name);

    if !Path::new(&full_path).exists() {
        download(Download {
            url,
            directory,
            file_name,
        });
    }
}

// --------------------------
// Checks if Directory Exist
// --------------------------
pub fn directory_exist(directory: &str) {
    // -------------------------
    // Check if directory exist
    // -------------------------
    if !Path::new(&directory).exists() {
        match create_dir_all(directory) {
            Ok(res) => res,
            Err(err) => std_error_exit!(format!("Failed to Create Directories : {}", err)),
        }
    }
}
