use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(_) => eprintln!("Go does not support project names"),
        None => {}
    }

    // Initialize default go project
    Command::new("go").arg("mod").arg("init").spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
