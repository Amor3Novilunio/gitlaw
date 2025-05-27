use super::read::directory_exist;
use crate::std_error_exit;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use std::{
    fs::File,
    io::{BufWriter, copy},
};

use super::types::Download;

// --------------
// Download File
// --------------
pub fn download(
    Download {
        directory,
        file_name,
        url,
    }: Download,
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

pub fn progress_bar(total_size: u64, file_name: &String) -> ProgressBar {
    // ----------------------
    // Progress Bar Initializer
    // ----------------------
    let progress_bar = ProgressBar::new(total_size);

    // ----------------------
    // Progress Bar Design
    // ----------------------
    let progress_bar_template = match ProgressStyle::with_template(
        "⏬ {msg}\n[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    ) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to Build Progress Bar Template : {}", err)),
    };

    // ----------------------
    // Progress Bar Setter
    // ----------------------
    progress_bar.set_style(progress_bar_template);
    progress_bar.set_message(format!("Downloading : {}", file_name));

    progress_bar
}
