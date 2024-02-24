use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::env::const::OS;

fn init(language: String, project_name: String) -> Result<(), std::io::Error> {
    let check = Command::new("which").arg("git").output();

    if check.is_err() {
        println!("Git is not installed!");
        println!("Would you like to install it? (y/n)");
        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
            //TODO: Install git
        } else {
            eprintln!("Git is required if you want to create a git repo.");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Git not installed")); 
        }
    }

    // Initialize git repository
    Command::new("git").arg("init").spawn().expect("Failed to initialize repository!");

    // Install OS Specific .gitignore
    let os_gitignore = include_str!(format!("os/{OS}.txt"));

    // Assume Language is already installed
    let lang_gitignore = include_str!(format!("langs/{language}.txt"));

    let gitignore = format!("{}\n{}", os_gitignore, lang_gitignore);

    Command::new("touch").arg(".gitignore").spawn().expect("Failed to create .gitignore");

    let command = Command::new("echo")
        .arg(gitignore)
        .arg(">>")
        .arg(".gitignore")
        .spawn()
        .expect("Failed to create .gitignore");

    // Creating a README.md
    Command::new("touch").arg("README.md").spawn().expect("Failed to create README.md");
    Command::new("echo").arg(format!("# {project_name}\n An example project in {language}")).spawn().expect("Failed to edit README.md");

    Ok(())
}

