use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind};

fn init() -> Result<(), Error> {
    let python: Result<_> = Command::new("which").arg("Python").output();

    if python.is_err() {
        print!("Would you like to install Python? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
           //TODO: Install Python
        } else {
            eprintln!("Python is required!");
            return Err(Error::new(ErrorKind::Other, "Python not installed"));
        }
    }

    // Setup venv
    Command::new("python3").arg("-m").arg("venv").arg("venv").spawn().expect("Failed to create virtual environment");
    Command::new("source").arg("venv/bin/activate").spawn().expect("Failed to activate virtual environment");

    // Install flask
    Command::new("pip").arg("install").arg("flask").spawn().expect("Failed to install flask");

    // Create Hello World
    Command::new("touch").arg("app.py").spawn().expect("Failed to create app.py");
    Command::new("echo").arg("from flask import Flask").arg(">>").arg("app.py").spawn().expect("Failed to edit app.py");
    Command::new("echo").arg("app = Flask(__name__)").arg(">>").arg("app.py").spawn().expect("Failed to edit app.py");
    
    Command::new("echo").arg("\n@app.route('/')").arg(">>").arg("app.py").spawn().expect("Failed to edit app.py");
    Command::new("echo").arg("def index():").arg(">>").arg("app.py").spawn().expect("Failed to edit app.py");
    Command::new("echo").arg("    return 'Hello, World!'").arg(">>").arg("app.py").spawn().expect("Failed to edit app.py");
    Ok(())
}
