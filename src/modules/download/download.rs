use super::download_helper::{directory_exist, progress_bar};
use crate::std_error_exit;
use reqwest::blocking::{Response, get};
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

use super::types;

// ----------------------------
// Entry Point For Downloading
// ----------------------------
pub fn new(
    types::New {
        directory,
        file_name,
        url,
    }: types::New,
) {
    // --------------------------
    // send request for download
    // --------------------------
    let mut response: Response = match get(&url) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Make HTTP Request : {}", err)),
    };

    // ----------------------
    // Create for Directories
    // ----------------------
    directory_exist(&directory);

    // ---------------
    // File Creation
    // ---------------
    let full_path = format!("{}/{}", &directory, file_name);

    let file: File = match File::create(&full_path) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to Create File : {}", err)),
    };

    // ------------------------------------
    // Get Total Size of the response file
    // ------------------------------------
    let total_size = match response.content_length() {
        Some(res) => res,
        None => std_error_exit!(format!(
            "Content Length of url provided not found : {} ",
            url
        )),
    };

    // -------------------------
    // Progress Bar Initializer
    // -------------------------
    let progress_bar = progress_bar(total_size, &file_name);

    // ----------------------
    // Optimize Write for performance
    // ----------------------
    let mut write_optimize = BufWriter::new(file);

    // --------------------------------------
    // Collect Downloaded Bytes to Chunk
    // --------------------------------------
    let mut collected_stream_bytes: u64 = 0;
    let mut buf: [u8; 32 * 1024] = [0u8; 32 * 1024];

    loop {
        // -------------------------------------------------
        // get a snapshot of the current stream byte chunk
        // -------------------------------------------------
        let download_snapshot = match response.read(&mut buf) {
            Ok(0) => {
                progress_bar.finish();
                break;
            }
            Ok(res) => res,
            Err(err) => std_error_exit!(format!("Failed to read from response stream : {}", err)),
        };

        // -----------------------------
        // Sum of stream bytes received
        // -----------------------------
        collected_stream_bytes += download_snapshot as u64;

        // ----------------------
        // Transfer the response data stream to write
        // ----------------------
        match write_optimize.write_all(&buf[..download_snapshot]) {
            // -----------------------------
            // progressbar position placement
            // -----------------------------
            Ok(_) => progress_bar.set_position(collected_stream_bytes),
            // ----------------------
            Err(err) => std_error_exit!(format!("Failed to Write File : {}", err)),
        };
    }
}
