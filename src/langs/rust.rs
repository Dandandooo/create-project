use std::process::Command;
use crate::CommandConfig;
use crate::ArgMap;

pub fn init(config: &CommandConfig) -> Result<(), String> {
    let args = match config.vars.get("name") {
        Some(name) => vec!["init", "--name", name],
        None => vec!["init"]
    };

    // Initialize default Cargo project
    if let Err(e) = Command::new("cargo").args(args) {
        return Err(format!("Failed to initialize cargo project: {}", e));
    }

    Ok(())
}

fn valid_args() -> ArgMap {
    Argmap::new()
}

