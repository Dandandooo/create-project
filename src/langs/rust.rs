use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::env::consts::OS;
use crate::ArgMap;
use crate::CommandConfig;


pub fn valid_args() -> ArgMap {
    todo!();
}

pub fn init(config: &CommandConfig) -> Result<(), String> {

    let cargo: Result<_> = Command::new("which")
        .arg("cargo")
        .output();
    
    if cargo.is_err() {
        print!("Would you like to install cargo? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
           install_cargo();
        } else {
            eprintln!("Cargo is required to initialize a rust project");
            return Err(Error::new(ErrorKind::Other, "Cargo not installed"));
        }
    }

    if let Err(_) = Command::new("which")
            .arg("cargo")
            .output() {
        print!("Would you like to install cargo? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
           install_cargo();
        } else {
            eprintln!("Cargo is required to initialize a rust project");
            return Err(format!("Cargo not installed"));
        }
    }

    
    if cargo.is_err() {
        print!("Would you like to install cargo? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
           install_cargo();
        } else {
            eprintln!("Cargo is required to initialize a rust project");
            return Err(Error::new(ErrorKind::Other, "Cargo not installed"));
        }
    }

    // Initialize default Cargo project
    Command::new("cargo")
        .arg("init")
        .spawn()
        .expect("Failed to initialize cargo project");

    Ok(())
}

fn install_cargo() {
    Command::new("curl")
        .arg("https://sh.rustup.rs")
        .arg("-sSf")
        .arg("|")
        .arg("sh")
        .spawn()
        .expect("Failed to install rustup");
}
