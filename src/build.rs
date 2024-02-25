use std::process::Command;
use std::fs::remove_dir_all;
use std::fs::create_dir;
use std::fs::write;

fn main() {
    let os_es = vec!["macos", "linux", "windows"];
    let ignores = vec![
        "angular,typescript,javascript".to_string(),
        "c".to_string(),
        "clojure".to_string(),
        "c++".to_string(),
        "dart".to_string(), 
        "django,python".to_string(),
        "dotnetcore".to_string(),
        "elixir".to_string(),
        "express,javascript,typescript".to_string(),
        "fastapi,python".to_string(),
        "flask,python".to_string(),
        "go".to_string(),
        "groovy".to_string(),
        "haskell".to_string(),
        "java".to_string(),
        "javascript".to_string(),
        "julia".to_string(),
        "kotlin".to_string(),
        "maven,java".to_string(),
        "nextjs,javascript,typescript".to_string(),
        "ocaml".to_string(),
        "perl".to_string(),
        "php,laravel".to_string(),
        "python".to_string(),
        "react,javascript".to_string(),
        "react,typescript".to_string(),
        "ruby,rails".to_string(),
        "rust".to_string(),
        "scala".to_string(),
        "svelte".to_string(),
        "swift".to_string(),
        "typescript".to_string(),
        "vue,javascript,typescript".to_string(),
        "zig".to_string(),
    ];

    if remove_dir_all("src/langs/gitignores").is_err() {
        eprintln!("Failed to remove gitignores directory");
    }

    create_dir("src/langs/gitignores").expect("Failed to create gitignores directory");


    for ignore in ignores {
        for os in os_es.iter() {
            install_gitignore(format!("{os},{ignore}"));
            println!("Installed gitignore for language: {ignore}");
        }
    }
}

fn install_gitignore(gitignore: String) {
    let filenames = gitignore.replace(",", "_");
    let file_path = format!("src/langs/gitignores/{filenames}.txt");
    let curl = Command::new("curl").arg(format!("https://www.toptal.com/developers/gitignore/api/{gitignore}")).output();
    let curl_text = String::from_utf8(curl.unwrap().stdout).unwrap();
    write(file_path, curl_text).expect("Failed to write to gitignore file");
}
