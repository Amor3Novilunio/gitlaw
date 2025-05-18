use std::env;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let git_command = Command::new("git");

    match args.get(0).map(String::as_str) {
        Some("commit") => {
            print!("hehe");
        }
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
