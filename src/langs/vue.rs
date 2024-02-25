use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(_) => eprintln!("Vue project name is the directory name!"),
        None => { }
    };

    // Initialize default vue project
    Command::new("npx").args(["create", "vue@latest"]).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}

