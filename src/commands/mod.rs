// Registry
pub mod commit;
pub mod motion;
pub mod summon;

// gitlaw

// Injection
use std::process::{Command, exit};

use gitlaw::std_error_exit;

pub fn passthrough(mut command: Command, args: Vec<String>) {
    match command.args(args).output() {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));

            if !output.status.success() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }

            exit(output.status.code().unwrap_or(1));
        }
        Err(err) => std_error_exit!(format!("gitlaw : Failed to run git: {}", err)),
    }
}
