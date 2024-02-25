use std::process::Command;
use crate::CommandConfig;
use crate::ArgMap;

pub fn init(config: &CommandConfig) -> Result<(), String> {
    let args = match config.vars.get("name") {
        Some(name) => vec!["init", "--name", name],
        None => vec!["init"]
    };

    // Initialize default Cargo project
    Command::new("cargo").args(args).spawn().expect("Failed to initialize cargo project");

    Ok(())
}

fn valid_args() -> ArgMap {
    Argmap::new()
}

