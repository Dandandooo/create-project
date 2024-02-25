use std::process::Command;
use std::error::Error;
use crate::CommandConfig;
use crate::ArgMap;

pub fn init(config: &CommandConfig) -> Result<(), Box<dyn Error>> {
    let args = match config.vars.get("name") {
        Some(name) => vec!["init", "--name", name],
        None => vec!["init"]
    };

    Command::new("cargo").args(args).spawn()?;
    Ok(())
}

fn valid_args() -> ArgMap {
    Argmap::new()
}

