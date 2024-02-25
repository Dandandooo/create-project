use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind}

fn init(directory: String, git: bool) {
    let npm = Command::new("which").arg("npm").output();
    
    if npm.is_err() {
        println!("npm is not installed");
        return;
    }


    
}
