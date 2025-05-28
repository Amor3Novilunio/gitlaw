// Registry
pub mod commit;
pub mod motion;
pub mod summon;

// Injection
use std::process::{Command, exit};

use gitlaw::std_error_exit;

// ------------
// Passthrough
// ------------
pub fn passthrough(mut command: Command, args: Vec<String>) {
    // ---------------------------------------------------------------------
    // run git command based on the env args -> [subcommand , flags, flags]
    // ---------------------------------------------------------------------
    match command.args(args).output() {
        Ok(output) => {
            // -----------------------------
            // store the response to buffer
            // -----------------------------
            print!("{}", String::from_utf8_lossy(&output.stdout));

            if !output.status.success() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            // --------------
            // flush on exit
            // --------------
            exit(output.status.code().unwrap_or(1));
        }
        Err(err) => std_error_exit!(format!("gitlaw : Failed to run git: {}", err)),
    }
}
