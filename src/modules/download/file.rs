use std::{
    fs::remove_file,
    io::{Write, stdin, stdout},
    path::Path,
};

use crate::{clear_terminal, std_error_exit};

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
    } else {
        // ---------------------------------------
        // ask user for rewrite the existing file
        // ---------------------------------------
        file_exist(types::New {
            directory,
            file_name,
            url,
        });
    }
}

// -------
// file exist do something or nothing
// -------
fn file_exist(
    types::New {
        directory,
        file_name,
        url,
    }: types::New,
) {
    println!("-------------------------");
    println!("--- File Already Exist --");
    println!("-------------------------\n");
    println!("-[ Y ] Rewrite ------------");
    println!("-------------------------");
    println!("-[ n ] Cancel -------------");

    // -------
    // prompt
    // -------
    let mut input = String::new();

    print!("\n How would you like to continue? : ");
    stdout().flush().unwrap();

    match stdin().read_line(&mut input) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed to Readline : {}", err)),
    };

    // ----------------
    // prompt matching
    // ----------------
    let input = input.trim();

    match input {
        "y" | "Y" => {
            clear_terminal!();
            match remove_file(format!("{}/{}", &directory, &file_name)) {
                Ok(_) => {}
                Err(err) => std_error_exit!(format!("Failed to Remove File : {}", err)),
            };
            check_file(types::New {
                directory,
                file_name,
                url,
            });
        }
        "n" | "N" => return,
        _ => {
            clear_terminal!();
            file_exist(types::New {
                directory,
                file_name,
                url,
            });
        }
    }
}
