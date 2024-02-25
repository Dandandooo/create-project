use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind};

fn init() -> Result<(), Error> {
    let node: Result<_> = Command::new("which").arg("npm").output();

    if node.is_err() {
        print!("Would you like to install Node.js? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
            // TODO: Install Node.js
        } else {
            eprintln!("Node.js is required!");
            return Err(Error::new(ErrorKind::Other, "Node.js not installed"));
        }
    }

    // Download Typescript
    let _ = Command::new("npm").arg("install").arg("-g").arg("typescript");
}
