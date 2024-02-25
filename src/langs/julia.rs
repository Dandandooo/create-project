use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind};

fn init() -> Result<(), Error> {
    let julia: Result<_> = Command::new("which").arg("julia").output();

    if julia.is_err() {
        print!("Would you like to install julia? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
            Command::new("curl")
                .arg("-fsSL")
                .arg("https://install.julialang.org")
                .arg("|")
                .arg("sh")
                .spawn()
                .expect("Failed to install julia");
        } else {
            eprintln!("Julia required!");
            return Err(Error::new(ErrorKind::Other, "Julia not installed"));
        }
    }

    // Initialize default Julia project
    Command::new("julia")
        .arg("--eval")
        .arg("using Pkg; Pkg.generate(\"HelloWorld\")")
        .spawn()
        .expect("Failed to initialize julia project");

    Command::new("mv")
        .arg("HelloWorld/*")
        .arg(".")
        .spawn()
        .expect("Failed to move files");

    Command::new("rm")
        .arg("-rf")
        .arg("HelloWorld")
        .spawn()
        .expect("Failed to remove HelloWorld directory");

    Ok(())
}
