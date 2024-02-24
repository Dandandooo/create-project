use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind};

fn init() -> Result<(), Error> {
    let check = Command::new("which").arg("ghcup").output();

    if check.is_err() {
        println!("GHCup not installed!");
        print!("Would you like to install GHCup? (y/n): ");
        let mut input = String::new();
        let _ = stdout().flush();  
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" || input.trim() == "Y" {
            Command::new("curl")
                .arg("--proto")
                .arg("'=https'")
                .arg("--tlsv1.2")
                .arg("-sSf")
                .arg("https://get-ghcup.haskell.org")
                .arg("|")
                .arg("sh")
                .spawn()
                .expect("Failed to install GHCup");
        } else {
            eprintln!("GHCup required for Haskell!");
            return Err(Error::new(ErrorKind::Other, "GHCup not installed"));
        }
    }

    // Create Sample Haskell Project
    Command::new("cabal").arg("init").spawn().expect("Failed to init Cabal Project");

    Ok(())
}
