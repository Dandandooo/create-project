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

fn parse_args() -> HashMap<String, String> {
    let mut arg_iter = args().collect().iter().skip(1);
    let mut map = HashMap::new();



    while let Some(arg) = arg_iter.next() {
        match arg {
            "-g" | "--git" => map.insert("git".to_string(), "true".to_string()),
            "-h" | "--help" => map.insert("help".to_string(), "true".to_string()),
            "-l" | "--list" => map.insert("list".to_string(), "true".to_string()),
            "-v" | "--version" => map.insert("version".to_string(), "true".to_string()),
            _ => {},
        },
        if arg.starts_with("--") {
            let key = arg.trim_start_matches("--");
            let value = arg_iter.next().unwrap_or("");
            map.insert(key.to_string(), value.to_string());
        }
    }
}
