// Registry
mod commands;
mod helpers;
mod modules;

// Injection
use std::{env, process::Command};

use commands::{commit, motion, summon};
// use gitlaw::modules::toml::read::config;
// use modules::download::{read, types};
// use modules::toml;

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

// !! Test
// test
//  create the logic flow in the maincheck_file
//  test run the llama.cpp and the model

fn main() {
    // let toml_config = toml::read::config();
    // read::check_file(types::Download {
    //     directory: "./".into(),
    //     file_name: toml_config.engine.file_name,
    //     url: toml_config.download.engine,
    // });

    let args: Vec<String> = env::args().skip(1).collect();

    let git_command = Command::new("git");

    match args.first().map(String::as_str) {
        Some("summon") => summon::run(args),
        Some("motion") => motion::run(),
        Some("commit") => commit::run(),
        Some(_) => commands::passthrough(git_command, args),
        None => std_error_exit!("Command not Found"),
    }
}
