use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res, Arg, ArgType };
use std::rc::Rc;

pub fn init(config: &CommandConfig) -> Res {
    let app_type = match config.args.get("library") {
        Some(_) => "groovy-library",
        None => "groovy-application"
    };

    let args = match config.vars.get("name") {
        Some(name) => vec!["init", "--type", app_type, "--project-name", name],
        None => vec!["init", "--type", app_type]
    };

    // Initialize default gradle project
    Command::new("gradle").args(args).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    let args = [
        Rc::new(Arg {
            name: "library".to_string(),
            description: "will create an library instead of an application".to_string(),
            aliases: string_set!["--lib"],
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
