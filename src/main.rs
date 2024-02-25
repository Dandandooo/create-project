use std::env::consts::OS;
use std::env::args;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <project name>", args[0]);
        return;
    }
    let project_name = &args[1];
    let directory = format!("{}/{}", env!("HOME"), project_name);
    let git = true;
    match init(directory, git) {
        Ok(_) => println!("Project initialized"),
        Err(e) => println!("Error: {}", e),
    }
}
