use std::process::Command;
use std::rc::Rc;
use std::collections::HashSet;
use crate::{ 
    CommandConfig, 
    ArgMap, 
    Arg, 
    Res,
    string_set,
    ArgType,
};

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
    let args = [
        Rc::new(Arg {
            name: "virtual-environment".to_string(),
            description: "will create a virtual environment using python3".to_string(),
            aliases: string_set!["v"],
            arg_type: ArgType::Flag,
            mutually_exclusive: HashSet::new(),
        }),
    ];

    let mut out = ArgMap::new();
    for arg in args {
        out.insert(arg.name.clone(), arg.clone());
    }
    out
}
