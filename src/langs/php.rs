use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

pub fn init(config: &CommandConfig) -> Res {
    let name = match config.vars.get("name") {
        Some(name) => name,
        None => "."
    };

    // Install PHP Composer (this looks really sus)
    Command::new("curl").arg("-s").arg("https://getcomposer.org/installer").arg("|").arg("php").arg("-").arg("--install-dir=/usr/local/bin").spawn()?;

    // Initialize default PHP project
    Command::new("composer").arg("create-project").arg("laravel/laravel").arg(name).spawn()?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
