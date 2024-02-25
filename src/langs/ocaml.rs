use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "."
    };

    // Install dune build system
    Command::new("opam").args(["install", "dune"]).spawn()?;

    // Set up ocaml project
    Command::new("dune").args(["init", "proj", name]).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
