use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind};

fn init() -> Result<(), Error> {
    let dart: Result<_> = Command::new("which").arg("dart").output();

    if dart.is_err() {
        print!("Would you like to install Dart? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
            //TODO: Install Dart
        } else {
            eprintln!("Dart is required!");
            return Err(Error::new(ErrorKind::Other, "Dart not installed"));
        }
    }

    // Initialize default dart project
    Command::new("dart").arg("create").arg(".").spawn().expect("Failed to initialize dart project");

    Ok(())
}
