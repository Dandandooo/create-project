use std::process::Command;
use crate::{ CommandConfig, ArgMap, Res };

use std::fs::{ File, create_dir, write };

pub fn init(config: &CommandConfig) -> Res {
    let dir = match config.vars.get("name") {
        Some(name) => {
            create_dir(name)?;
            name
        },
        None => "."
    };
    let sample_code = b"#!/usr/bin/perl\nuse warnings;\nprint(\"Hello, World!\\n\");";

    write(format!("{}/main.pl", name), sample_code)?;

    Ok(())
}

pub fn valid_args() -> ArgMap {
    ArgMap::new()
}
