use std::process::Command;
use std::io::{stdin, stdout, Write};

fn init(directory: String, git: bool) -> Result<(), std::io::Error> {
    let cargo: Result<_> = Command::new("which")
        .arg("cargo")
        .output();
    
    if cargo.is_err() {
        println!("Would you like to install cargo? (y/n)");
        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
           install_cargo();
        } else {
            println!("Cargo is required to initialize a rust project");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Cargo not installed"));
        }
    }

    if git {
        Command::new("git")
            .arg("init")
            .spawn()
            .expect("Failed to initialize git repository");

        let rust_gitignore = include_str!("rust.txt");

        let command = Command::new("echo")
            .arg(rust_gitignore)
            .arg(">>")
            .arg(".gitignore")
            .spawn()
            .expect("Failed to create .gitignore");

        let command = Command::new("echo")
            .arg(rust_gitignore)
            .arg(">>")
            .arg(".gitignore")
            .spawn()
            .expect("Failed to create .gitignore");
    }

    Command::new("cargo")
        .arg("init")
        .arg(directory)
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
