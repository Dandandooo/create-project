pub mod args;
pub mod langs;

use std::env::args;
use std::collections::HashMap;
use args::Arg;


struct CommandConfig {
    args: HashMap<Arg, String>,

}

fn parse_args(args: std::env::Args) -> HashSet<Arg, Value> {
    let mut args = args.into_iter().skip(1);
    let mut map = HashMap::new();

    // while let Some(arg) = args.next() {
    //     match arg {
    //         "-g" | "--git" => {  
    //             map.insert("git".to_string(), "true".to_string());
    //         },
    //         "-h" | "--help" =>{
    //             map.insert("help".to_string(), "true".to_string());
    //         },
    //         "-l" | "--list" => {
    //             map.insert("list".to_string(), "true".to_string());
    //         },
    //         "-v" | "--version" => {
    //             map.insert("version".to_string(), "true".to_string());
    //         },
    //         _ => {},
    //     }
    //
    //     if arg.starts_with("--") {
    //         let key = arg.trim_start_matches("--");
    //         let value = arg_iter.next().unwrap_or("");
    //         map.insert(key.to_string(), value.to_string());
    //     }
    // }
    map
}
