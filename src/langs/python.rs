use std::process::Command;
use std::io::{stdin, stdout, Write};

fn init() -> Result<(), std::io::Error>{
    let check = Command::new("which")
        .arg("python3")
        .output();

    if check.is_err() {
        println!("Python is not installed!");
        println!("Would you like to install it? (y/n)");
        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
            //TODO: Install python
        } else {
            println!("Python is required to use this tool.");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Python not installed"));
        }
    }

    let python = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .spawn()
        .expect("Failed to create virtual environment");

    // Create Hello World
    Command::new("touch").arg("main.py").spawn().expect("Failed to create main.py");
    Command::new("echo").arg("print(\"Hello, World!\")").arg(">>").arg("main.py").spawn().expect("Failed to edit main.py");

    Ok(())
}
