use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(_) => eprintln!("Flask does not support project names"),
        None => {}
    }

    // Setup venv
    Command::new("python3").args(["-m", "venv", "venv"]).spawn()?;

    // Install flask
    Command::new("venv/bin/pip").args(["install", "flask"]).spawn()?;

    // Create Hello World
    let sample_code = b"from flask import Flask\napp = Flask(__name__)\napp = Flask(__name__)\n\n@app.route('/')\ndef index():\n    return 'Hello, World!'".to_string();
    
    std::fs::write("main.py", sample_code)?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
