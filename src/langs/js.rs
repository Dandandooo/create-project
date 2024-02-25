use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

use std::fs::{ create_dir, rename, remove_dir, write };

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(name) => {
            Command::new("npm").args(["init", "-y", "-w", name]).spawn()?;
            rename(format!("{name}/package.json"), "package.json")?;
            remove_dir(name)?;
        },
        None => {
            Command::new("npm").arg("init").arg("-y").spawn()?;
        }
    } 
    // Create src directory
    create_dir("src")?;

    // Create index.js
    write("src/index.js", b"console.log('Hello, World!')")?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
