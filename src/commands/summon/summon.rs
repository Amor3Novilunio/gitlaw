use super::engine::summon_engine;
use super::model::summon_model;
use super::summon_flags::validate_flags;
use super::types::Flags;

use gitlaw::helpers::commands::skip_sub_command;

// ------------------------------------------
// summon Command
// aka Download Based on Toml Configuration
// ------------------------------------------
pub fn run(args: Vec<String>) {
    // ----------------------------
    // remove summon from the args
    // ----------------------------
    let args: Vec<String> = skip_sub_command(args);

    // ------------------------------------
    // if no flags detected | download both
    // ------------------------------------
    if args.is_empty() {
        // trigger download for all;
        summon_engine();
        println!("\n");
        summon_model();
    }

    // ------------
    // Flag Checker
    // ------------
    let mut flags: Flags = Flags {
        engine: false,
        model: false,
        help: false,
        invalid: false,
    };

    // --------------------
    // Update flags based on args
    // --------------------
    for flag in args {
        match flag.as_str() {
            "-h" | "--help" => flags.help = true,
            "-e" => flags.engine = true,
            "-m" => flags.model = true,
            _ => flags.invalid = true,
        }
    }

    // ----------------
    // Flag Initializer
    // ----------------
    validate_flags(flags);
}
