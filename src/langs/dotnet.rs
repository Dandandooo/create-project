use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res, Arg, ArgType };
use std::rc::Rc;

pub fn init(config: &CommandConfig) -> Res {
    let mut args = vec!["new"];
    
    match config.vars.get("template") {
        Some(template) => args.push(template),
        None => args.push("console")
    }

    match config.vars.get("name") {
        Some(name) => args.push("-n").push(name),
        None => {} 
    }

    match config.vars.get("language") {
        Some(lang) => args.push("--lang").push(lang),
        None => {}
    }

    match config.vars.get("framework") {
        Some(framework) => args.push("--framework").push(framework),
        None => {}
    }

    // Initialize dotnet project
    Command::new("dotnet").args(args).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    let args = [
        Rc::new(Arg {
            name: "language".to_string(),
            description: "The language for .NET to use: C# (default), F#, VB".to_string(),
            aliases: string_set!["--lang"],
            arg_type: ArgType::Value,
            mutually_exclusive: std::collections::HashSet::new(),
        }),
        Rc::new(Arg {
            name: "template".to_string(),
            description: "The template to use for the project. Default is Console Application. More info at https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-new.".to_string(),
            aliases: string_set!["--template"],
            arg_type: ArgType::Value,
            mutually_exclusive: std::collections::HashSet::new(),
        }),
        Rc::new(Arg {
            name: "framework".to_string(),
            description: "The target framework for the project. Optional. More info at https://docs.microsoft.com/en-us/dotnet/standard/frameworks.".to_string(),
            aliases: string_set!["--framework"],
            arg_type: ArgType::Value,
            mutually_exclusive: std::collections::HashSet::new(),
        }),
    ];

    let mut out = ArgMap::new();
    for arg in args.iter() {
        out.insert(arg.name.clone(), arg.clone());
    }
    out
}
