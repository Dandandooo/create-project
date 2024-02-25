use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "."
    };

    // Initialize default maven project
    Command::new("mvn").args(["archetype:generate", "-DgroupId=com.mycompany.app", "-DartifactId=my-app", "-DarchetypeArtifactId=maven-archetype-quickstart", "-DinteractiveMode=false"]).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
