use std::process::Command;
use std::collections::HashSet;

#[path = "langs/mod.rs"]
mod langs;

use langs::supported_languages;

fn main() {
    let ignores = compile_ignores();

    for ignore in ignores {
        install_gitignore(ignore);
        println!("Installed gitignore for language: {ignore}");
    }
}

fn compile_ignores() -> Vec<HashSet<String>> {
    let mut ignores = Vec::new();

    let langs = supported_languages();
    for (lang, properties) in langs.into_iter() {
        for ignore in properties.get("ignore").unwrap() {
            ignores.push(ignore.clone());
        }
    }

    ignores
}

fn install_gitignore(set: HashSet<String>) {
    let sorted_filenames = set.into_iter().collect::<Vec<String>>();
    sorted_filenames.sort();
    let gitignore = sorted_filenames.join(",");
    let filenames = sorted_filenames.join("_"); 

    Command::new("touch").args([format!("./langs/gitignores/{filenames}.txt")]).spawn().expect("Failed to install gitignore");
    Command::new("curl").args([format!("https://www.toptal.com/developers/gitignore/api/{gitignore}"), ">".to_string(), format!("./langs/gitignores/{filenames}.txt")]).spawn().expect("Failed to install gitignore");
}
