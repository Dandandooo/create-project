use std::process::Command;
use crate::langs::supported_languages;
use std::collections::HashSet;

fn main() {
    let ignores = compile_ignores();

    for ignore in ignores {
        install_gitignore(ignore);
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
    let gitignore = set.join(",");

    let sorted_filenames = set.into_iter().collect::<Vec<String>>();
    sorted_filenames.sort();
    let filenames = sorted_filenames.join(","); 

    Command::new("curl").args([format!("https://www.toptal.com/developers/gitignore/api/{gitignore}", ">", format!("./langs/gitignores/{filenames}.txt")]).spawn().expect("Failed to install gitignore");
}
