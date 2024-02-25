use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res, Arg, ArgType, string_set };
use std::rc::Rc;

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(_) => eprintln!("Flask does not support project names"),
        None => {}
    }
    
    let venv_prefix = match config.vars.get("virtual-environment") {
        Some(_) => {
            Command::new("python3").args(["-m", "venv", "venv"]).spawn()?;
            "venv/bin/"
        },
        None => "",
    };

    // Install flask
    Command::new(format!("{venv_prefix}pip")).args(["install", "flask"]).spawn()?;

    // Create Hello World
    let sample_code = b"from flask import Flask\napp = Flask(__name__)\napp = Flask(__name__)\n\n@app.route('/')\ndef index():\n    return 'Hello, World!'";
    
    std::fs::write("app.py", sample_code)?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    let args = [
        Rc::new(Arg {
            name: "virtual-environment".to_string(),
            description: "will create a virtual environment using python3".to_string(),
            aliases: string_set!["-v"],
            arg_type: ArgType::Flag,
            mutually_exclusive: std::collections::HashSet::new(),
        }),
    ];

    let mut out = ArgMap::new();
    for arg in args.iter() {
        out.insert(arg.name.clone(), arg.clone());
    }
    out
}
