use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::env::consts::OS;
use crate::ArgMap;
use crate::CommandConfig;


pub fn valid_args() -> ArgMap {
    todo!();
}

pub fn init(config: &CommandConfig) -> Result<(), String> {
    todo!();
    // let cargo: Result<_> = Command::new("which")
    //     .arg("cargo")
    //     .output();
    // 
    // if cargo.is_err() {
    //     println!("Would you like to install cargo? (y/n)");
    //     let mut input = String::new();
    //     print!("> ");
    //     let _ = stdout().flush();
    //     stdin().read_line(&mut input).unwrap();
    //     if input.trim() == "y" {
    //         Command::new("curl")
    //             .arg("https://sh.rustup.rs")
    //             .arg("-sSf")
    //             .arg("|")
    //             .arg("sh")
    //             .spawn()
    //             .expect("Failed to install rustup");
    //     } else {
    //         println!("Cargo is required to initialize a rust project");
    //         return Err(std::io::Error::new(std::io::ErrorKind::Other, "Cargo not installed"));
    //     }
    // }
    //
    // if git {
    //     Command::new("git")
    //         .arg("init")
    //         .spawn()
    //         .expect("Failed to initialize git repository");
    //     let ignore: Result<_> = Command::new("curl")
    //         .arg(format!("https://www.toptal.com/developers/gitignore/api/{OS},rust"))
    //         .arg("-output")
    //         .arg(".gitignore")
    //         .current_dir(directory)
    //         .output();
    //
    //     if ignore.is_err() {
    //         println!("Failed to download .gitignore file! Creating empty .gitignore file...");
    //         Command::new("touch")
    //             .arg(".gitignore")
    //             .current_dir(directory)
    //             .spawn()
    //             .expect("Failed to create .gitignore file")
    //     } else {
    //         println!("Downloaded .gitignore file");
    //     } 
    //
    // }
    //
    // Command::new("cargo")
    //     .arg("init")
    //     .arg(directory)
    //     .spawn()
    //     .expect("Failed to initialize cargo project");

    Ok(())
}
