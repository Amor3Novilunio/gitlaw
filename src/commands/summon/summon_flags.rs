use super::engine::summon_engine;
use super::model::summon_model;
use super::summon_helper::help;
use super::types::Flags;

// -----------------------
// Initialize Valid Flags
// -----------------------
pub fn validate_flags(flags: Flags) {
    // ------------
    // -h | --help
    // ------------
    if flags.help == true {
        help("Gitlaw Summon Flags\n".into());
        return;
    }

    // ------------
    // -e | engine
    // ------------
    if flags.engine == true {
        summon_engine();
    }

    // ------------
    // -e | model
    // ------------
    if flags.model == true {
        summon_model();
    }

    // ---------------------------------
    // flag from args is not registered
    // ---------------------------------
    if flags.invalid == true {
        help("Unknown Flag Detected\n".into());
    }
}
