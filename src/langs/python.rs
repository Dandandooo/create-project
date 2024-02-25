use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(_) => eprintln!("Python does not support project names"),
        None => {}
    }

    // Create virtual environment
    Command::new("python3").args(["-m", "venv", "venv"]).spawn()?; 

    // Create Hello World
    std::fs::write("main.py", b"print(\"Hello, World!\")")?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
