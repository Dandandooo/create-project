use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res, Arg, ArgType, string_set };
use std::rc::Rc;

pub fn init(config: &CommandConfig) -> Res {
    let mut args = vec!["package", "init", "--type"];
    match config.vars.get("app_type") {
        Some(app_type) => match app_type.as_str() {
            "library" => args.push("library"),
            "executable" => args.push("executable"),
            "tool" => args.push("tool"),
            "build-tool-plugin" => args.push("build-tool-plugin"),
            "command-plugin" => args.push("command-plugin"),
            "macro" => args.push("macro"),
            "empty" => args.push("empty"),
            _ => args.push("library")
        },
        None => args.push("library")
    };

    match config.vars.get("name") {
        Some(name) => {
            args.push("--name");
            args.push(name)
        },
        None => {}
    }

    // Initialize swift project
    Command::new("swift").args(args).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    let args = [
        Rc::new(Arg {
            name: "app_type".to_string(),
            description: "Do you want to create a(n): library(default), executable, tool, build-tool-plugin, command-plugin, macro, empty".to_string(),
            aliases: string_set!["--type"],
            arg_type: ArgType::Var{
                parse: Box::new(|s| { Ok(s.to_string()) }),
            },
            mutually_exclusive: std::collections::HashSet::new(),
        }),
    ];

    let mut out = ArgMap::new();
    for arg in args.iter() {
        out.insert(arg.name.clone(), arg.clone());
    }
    out
}
