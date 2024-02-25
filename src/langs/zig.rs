use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res, Arg, ArgType };
use std::rc::Rc;

pub fn init(config: &CommandConfig) -> Res {
    match config.vars.get("name") {
        Some(_) => eprintln!("Name is not a valid argument for zig"),
        None => {}
    }

    let app_type = match config.vars.get("type") {
        Some(app_type) => match app_type {
            "exe" => "exe",
            "lib" => "lib",
            _ => "exe"
        },
        None => "exe"
    };

    // Initialize default zig project
    Command::new("zig").arg(format!("init-{app_type}")).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    let args = [
        Rc::new(Arg {
            name: "type".to_string(),
            description: "The type of application to create (exe or lib)".to_string(),
            aliases: string_set!["-t", "--type"],
            arg_type: ArgType::Var,
            mutually_exclusive: std::collections::HashSet::new(),
        }),
    ];

    let mut out = ArgMap::new();
    for arg in args.iter() {
        out.insert(arg.name.clone(), arg.clone());
    }
    out
}
