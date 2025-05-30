use crate::std_error_exit;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::create_dir_all, path::Path};

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

// --------------------------
// Progress bar for download
// --------------------------
pub fn progress_bar(total_size: u64, file_name: &String) -> ProgressBar {
    // ----------------------
    // Progress Bar Initializer
    // ----------------------
    let progress_bar = ProgressBar::new(total_size);

    // ----------------------
    // Progress Bar Design
    // ----------------------
    let progress_bar_template = match ProgressStyle::with_template(
        "⏬ {msg}\n[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})\n",
    ) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to Build Progress Bar Template : {}", err)),
    };

    // ----------------------
    // Progress Bar Setter
    // ----------------------
    progress_bar.set_style(progress_bar_template);
    progress_bar.set_message(format!("Downloading : {}\n", file_name));

    progress_bar
}
