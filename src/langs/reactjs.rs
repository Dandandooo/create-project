use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "."
    };

    // Initialize default react project
    Command::new("npx").arg("create-react-app").arg(name).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
