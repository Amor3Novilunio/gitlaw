// Injection
use std::{env, process::Command};

use gitlaw::std_error_exit;

use super::commands::{commit, motion, passthrough, summon};

// flow
// initial check on toml
// check for models & mode what to use
// validate path or api url
// match pattern applied when they do commit
// gitlaw commit
// run the ai model -> compiles all messages from diff -> creates a short readable response message (for new let it be whatever once its working we can apply the rules of conventional result of this message)
// provide the user response to re-generate the proceed with the provided result or cancel the commit
// once all the above are done we can proceed with
// after proceeding generate a md file based on law sequence
// inside this file will contain a before and after based on what you changed
// it will look like this
// line number from which something changed
// before
// code from before
// after
// code from after

// !! todo
// !! summon engine and download integration
// !! create setup command for toml creation
// !! start with commit & test the model there if it is working as intended

// ----------------------------
// Sub Command Matching
// ----------------------------
pub fn init() {
    // -----------------------------
    // Skip gitlaw from env::args()
    // -----------------------------
    let args: Vec<String> = env::args().skip(1).collect();

    // ------------
    // Git Command
    // ------------
    let git_command = Command::new("git");

    // ---------------------
    // Sub command Matching
    // ---------------------
    match args.first().map(String::as_str) {
        Some("summon") => summon::run(args),
        Some("motion") => motion::run(),
        Some("commit") => commit::run(),
        Some(_) => passthrough(git_command, args),
        None => std_error_exit!("Command not Found"),
    }
}
