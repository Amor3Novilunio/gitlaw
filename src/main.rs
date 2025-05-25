use std::{
    env,
    process::{Command, exit},
};
mod helpers;
mod modules;
use modules::{engine::download::download_engine, toml::read::settings};

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

// !! new REq
// reqwest for downloading
//  need to think about how the flow works for this one 

fn main() {
    // let toml_settings = settings();
    download_engine("url", "destination");

    let args: Vec<String> = env::args().skip(1).collect();

    let git_command = Command::new("git");

    match args.get(0).map(String::as_str) {
        Some("commit") => println!("Pending"),
        Some(_) => passthrough(git_command, args),
        None => {
            eprintln!("command not found");
            exit(1);
        }
    }
}

fn passthrough(mut command: Command, args: Vec<String>) {
    match command.args(args).output() {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));

            if !output.status.success() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }

            exit(output.status.code().unwrap_or(1));
        }
        Err(err) => {
            eprint!("gitlaw : Failed to run git: {}", err);
            exit(1);
        }
    }
}
