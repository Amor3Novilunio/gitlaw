// gitlaw summon [OPTIONS]

// 🔮 Summon the powers Gitlaw needs to operate.
// You may summon the engine, model, or both.

// OPTIONS:
//   -e, --engine     Summon the Gitlaw engine
//   -m, --model      Summon the AI model
//   -h, --help       Show this mystical guide

// use std::process::Command;

use gitlaw::{dprintln, std_error_exit};

pub fn run(args: Vec<String>) {
    // flags
    dprintln!(args);

    // and for this we dont need git argument we only need the flags

    if args.is_empty() {
        help();
    }

    for flag in args {
        if flag.contains("-h") || flag.contains("--help") {
            help();
        }

        match flag.as_str() {
            "-e" | "--engine" => summon_engine(),
            "-m" | "--model" => summon_model(),
            "-h" | "--help" => help(),
            _ => continue,
        }
    }
}

fn help() {
    std_error_exit!(
        "
         Unknown Flag Detected.\n\n\
             OPTIONS:\n  \
               -e, --engine     Download the Gitlaw engine\n  \
               -m, --model      Download the AI model\n  \
               -h, --help       Show this mystical guide"
    );
}

fn summon_engine() {
    println!("BEEP BOOP BAAP ENGINE");
}
fn summon_model() {
    println!("BEEP BOOP BAAP Model");
}
