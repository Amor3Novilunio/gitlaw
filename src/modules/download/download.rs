use super::download_helper::{directory_exist, progress_bar};
use crate::std_error_exit;
use reqwest::blocking::get;
use std::{
    fs::File,
    io::{BufWriter, copy},
};

use super::types;

// --------------
// Download File
// --------------
pub fn new(
    types::Download {
        directory,
        file_name,
        url,
    }: types::Download,
) {
    // ----------------------
    // send request for download
    // ----------------------
    let mut response = match get(&url) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Make HTTP Request : {}", err)),
    };

    // ----------------------
    // Create for Directories
    // ----------------------
    directory_exist(&directory);

    // ----------------------
    // File Creation
    // ----------------------
    let full_path = format!("{}/{}", &directory, file_name);

    let file = match File::create(&full_path) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to Create File : {}", err)),
    };

    // ----------------------
    // Progress Bar
    // ----------------------
    let total_size = match response.content_length() {
        Some(res) => res,
        None => std_error_exit!(format!(
            "Content Length of url provided not found : {} ",
            url
        )),
    };

    let progress_bar = progress_bar(total_size, &file_name);

    // ----------------------
    // Optimize Write for performance
    // ----------------------
    let mut write_optimize = BufWriter::new(file);

    // ----------------------
    // Transfer the response data stream to write
    // ----------------------
    match copy(&mut response, &mut write_optimize) {
        Ok(_) => progress_bar.finish_with_message(format!("Download Complete : {}", file_name)),
        Err(err) => std_error_exit!(format!("Failed to Write File : {}", err)),
    }
}
