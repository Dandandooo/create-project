use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };


// ANGULAR IS ONLY IN TYPESCRIPT
pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "."
    };

    // Initialize default angular project
    Command::new("npx").args(["-p", "@angular/cli", "ng", "new", name]).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
