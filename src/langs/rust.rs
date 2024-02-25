use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    let args = match config.vars.get("name") {
        Some(name) => vec!["init", "--name", name],
        None => vec!["init"]
    };

    Command::new("cargo").args(args).spawn()?;
    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
