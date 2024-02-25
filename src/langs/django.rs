use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res, Arg, ArgType };
use std::rc::Rc;

pub fn init(config: &CommandConfig) -> Res {
    let venv_prefix = match config.vars.get("virtual-environment") {
        Some(_) => {
            Command::new("python3").args(["-m", "venv", "venv"]).spawn()?;
            "venv/bin/"
        },
        None => ""
    };
    Command::new(format!("{venv_prefix}pip")).args(["install", "Django"]).spawn()?;

    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "."
    };

    Command::new(format!("{venv_prefix}django-admin")).args(["startproject", "project", name]).spawn()?;

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

