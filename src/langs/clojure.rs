use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "."
    };

    // Initialize default clojure project => Requires leiningen package manager
    Command::new("lein").args(["-X", "new", "app", name]).spawn()?;
    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
