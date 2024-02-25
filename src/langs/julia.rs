use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "HelloWorld"
    }; 
    
    // Initialize default Julia project
    Command::new("julia")
        .arg("--eval")
        .arg(format!("using Pkg; Pkg.generate(\"{name}\")"))
        .spawn()?;

    Command::new("mv").args([format!("{name}/*"), ".".to_string()]).spawn()?;

    std::fs::remove_dir(name)?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
