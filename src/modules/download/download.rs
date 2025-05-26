use reqwest::blocking::get;
use std::{
    fs::{File, create_dir_all},
    io::{BufWriter, copy},
};

use crate::std_error_exit;

pub fn download(url: &str, directory: &str, file_name: &str) {
    // ----------------------
    // send request for download
    // ----------------------
    let mut response = match get(url) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Make HTTP Request : {}", err)),
    };

    // ----------------------
    // Create for Directories
    // ----------------------
    match create_dir_all(directory) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to Create Directories : {}", err)),
    };

    // ----------------------
    // File Creation
    // ----------------------
    let full_path = format!("{}/{}", directory, file_name);

    let file = match File::create(&full_path) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to Create File : {}", err)),
    };

    // ----------------------
    // Optimize Write for performance
    // ----------------------
    let mut write_optimize = BufWriter::new(file);

    // ----------------------
    // Transfer the response data stream to write
    // ----------------------
    match copy(&mut response, &mut write_optimize) {
        Ok(_) => println!("File Saved Successful"),
        Err(err) => std_error_exit!(format!("Failed to Write File : {}", err)),
    }
}
