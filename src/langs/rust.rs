use std::process::Command;
use std::io::{stdin, stdout, Write};

fn init() -> Result<(), std::io::Error> {
    let cargo: Result<_> = Command::new("which")
        .arg("cargo")
        .output();
    
    if cargo.is_err() {
        println!("Would you like to install cargo? (y/n)");
        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
           install_cargo();
        } else {
            eprintln!("Cargo is required to initialize a rust project");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Cargo not installed"));
        }
    }

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
