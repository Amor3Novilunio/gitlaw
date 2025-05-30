use super::download_helper::{directory_exist, progress_bar};
use crate::{dprintln, std_error_exit};
use reqwest::blocking::{Response, get};
use std::{
    fs::File,
    io::{BufWriter, Read, copy},
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
    // !! new is only a route a compilation of concerns
    // !! flow
    // !! send the request and get the response
    // !! if request safe proceed
    // !! start checking directory and create it if missing
    // !!

    // --------------------------
    // send request for download
    // --------------------------
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

    // --------------------------------------
    // Collect Download Data By Stream Chunk
    // --------------------------------------
    let mut buf: [u8; 32 * 1024] = [0u8; 32 * 1024];

    let download_snapshot = match response.read(&mut buf) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to read from response stream : {}", err)),
    };


    // -------------------------
    // Progress Bar For Writing
    // -------------------------
    let write_progress_bar = progress_bar(total_size, &file_name);

    // // ----------------------
    // // Optimize Write for performance
    // // ----------------------
    // let mut write_optimize = BufWriter::new(file);

    // // ----------------------
    // // Transfer the response data stream to write
    // // ----------------------
    // match copy(&mut response, &mut write_optimize) {
    //     Ok(_) => {
    //         write_progress_bar.finish_with_message(format!("Download Complete : {}", file_name))
    //     }
    //     Err(err) => std_error_exit!(format!("Failed to Write File : {}", err)),
    // }
}
