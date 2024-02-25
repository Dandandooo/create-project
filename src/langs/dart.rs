use std::process::Command;
use std::error::Error;
use crate::CommandConfig;
use crate::ArgMap;

pub fn init(config: &CommandConfig) -> Result<(), Box<dyn Error>> {
    match config.vars.get("name") {
        Some(_) => eprintln!("Dart does not support project names"),
        None => {}
    }

    // Initialize default dart project
    Command::new("dart").arg("create").arg(".").spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    Argmap::new()
}
