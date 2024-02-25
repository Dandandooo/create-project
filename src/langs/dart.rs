use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
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
