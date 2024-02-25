use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res, Arg, ArgType };
use std::rc::Rc;

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(name) => eprintln!("FastAPI does not support project names"),
        None => {};
    }

    let venv_prefix = match config.vars.get("virtual-environment") {
        Some(_) => {
            Command::new("python3").args(["-m", "venv", "venv"]).spawn()?;
            "venv/bin/"
        },
        None => ""
    };

    Command::new(format!("{venv_prefix}pip")).args(["install", "fastapi"]).spawn()?;

    let sample_code = b"from fastapi import FastAPI\napp = FastAPI()\n\napp = FastAPI()\n\n@app.get('/')\nasync def root():\n    return {\"message\": \"Hello, World\"}";

    std::fs::write("main.py", sample_code)?;

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
