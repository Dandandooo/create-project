use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind};

fn init() -> Result<(), Error> {
    let go: Result<_> = Command::new("which").arg("go").output();

    if go.is_err() {
        print!("Would you like to install Go? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
            //TODO: Install Go
        } else {
            eprintln!("Go is required!");
            return Err(Error::new(ErrorKind::Other, "Go not installed"));
        }
    }

    // Initialize default go project
    Command::new("go").arg("mod").arg("init").spawn().expect("Failed to initialize go project");

    Ok(())
}
