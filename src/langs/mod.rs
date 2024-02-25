mod angular;
mod c;
mod clojure;
mod cpp;
mod dart;
mod django;
mod dotnet;
mod elixir;
mod express;
mod fastapi;
mod flask;
mod go;
mod groovy;
mod haskell;
mod java;
mod javascript;
mod julia;
mod kotlin;
mod maven;
mod nextjs;
mod ocaml;
mod perl;
mod php;
mod python;
mod reactjs;
mod reactts;
mod ruby;
mod rust;
mod scala;
mod svelte;
mod swift;
mod typescript;
mod vue;
mod zig;

use crate::Res;
use super::args::{CommandConfig, ArgMap};
use std::collections::{HashSet, HashMap};
use std::env;
use std::rc::Rc;
use super::string_set;

pub struct Language {
    pub name: String,
    pub exec: Box<dyn Fn(&CommandConfig) -> Res>,
    pub valid_args: Box<dyn Fn() -> ArgMap>,
    pub uses: HashSet<String>, // dependencies
    pub ignores: HashSet<String>, // git ignores
}

pub fn supported_languages() -> HashMap<String, Rc<Language>> {
    // Alphabetically sorted
    let langs = [
        Rc::new(Language {
            name: "angular".to_string(),
            exec: Box::new(angular::init),
            valid_args: Box::new(angular::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "angular", "typescript", "javascript"],
        }),

        Rc::new(Language {
            name: "c".to_string(),
            exec: Box::new(c::init),
            valid_args: Box::new(c::valid_args),
            uses: string_set!["clang", "make"],
            ignores: string_set![env::consts::OS, "c"],
        }),

        Rc::new(Language {
            name: "clojure".to_string(),
            exec: Box::new(clojure::init),
            valid_args: Box::new(clojure::valid_args),
            uses: string_set!["clojure", "leiningen"],
            ignores: string_set![env::consts::OS, "clojure"],
        }),

        Rc::new(Language {
            name: "cpp".to_string(),
            exec: Box::new(cpp::init),
            valid_args: Box::new(cpp::valid_args),
            uses: string_set!["clang++", "make", "cmake"],
            ignores: string_set![env::consts::OS, "cpp"],
        }),

        Rc::new(Language {
            name: "dart".to_string(),
            exec: Box::new(dart::init),
            valid_args: Box::new(dart::valid_args),
            uses: string_set!["dart"],
            ignores: string_set![env::consts::OS, "dart"],
        }),

        Rc::new(Language {
            name: "django".to_string(),
            exec: Box::new(django::init),
            valid_args: Box::new(django::valid_args),
            uses: string_set!["python3"],
            ignores: string_set![env::consts::OS, "django", "python"],
        }),

        Rc::new(Language {
            name: "dotnet".to_string(),
            exec: Box::new(dotnetcore::init),
            valid_args: Box::new(dotnetcore::valid_args),
            uses: string_set!["dotnet"],
            ignores: string_set![env::consts::OS, "dotnetcore"],
        }),

        Rc::new(Language {
            name: "elixir".to_string(),
            exec: Box::new(elixir::init),
            valid_args: Box::new(elixir::valid_args),
            uses: string_set!["elixir"],
            ignores: string_set![env::consts::OS, "elixir"],
        }),

        Rc::new(Language {
            name: "express".to_string(),
            exec: Box::new(express::init),
            valid_args: Box::new(express::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "express", "javascript", "typescript"],
        }),

        Rc::new(Language {
            name: "fastapi".to_string(),
            exec: Box::new(fastapi::init),
            valid_args: Box::new(fastapi::valid_args),
            uses: string_set!["python3"],
            ignores: string_set![env::consts::OS, "fastapi", "python"],
        }),

        Rc::new(Language {
            name: "flask".to_string(),
            exec: Box::new(flask::init),
            valid_args: Box::new(flask::valid_args),
            uses: string_set!["python3"],
            ignores: string_set![env::consts::OS, "flask", "python"],
        }),

        Rc::new(Language {
            name: "go".to_string(),
            exec: Box::new(go::init),
            valid_args: Box::new(go::valid_args),
            uses: string_set!["go"],
            ignores: string_set![env::consts::OS, "go"],
        }),

        Rc::new(Language {
            name: "groovy".to_string(),
            exec: Box::new(groovy::init),
            valid_args: Box::new(groovy::valid_args),
            uses: string_set!["groovy", "gradle"],
            ignores: string_set![env::consts::OS, "groovy"],
        }),

        Rc::new(Language {
            name: "haskell".to_string(),
            exec: Box::new(haskell::init),
            valid_args: Box::new(haskell::valid_args),
            uses: string_set!["ghcup"],
            ignores: string_set![env::consts::OS, "haskell"],
        }),

        Rc::new(Language {
            name: "java".to_string(),
            exec: Box::new(java::init),
            valid_args: Box::new(java::valid_args),
            uses: string_set!["java", "gradle"],
            ignores: string_set![env::consts::OS, "java"],
        }),

        Rc::new(Language {
            name: "javascript".to_string(),
            exec: Box::new(javascript::init),
            valid_args: Box::new(javascript::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "javascript"],
        }),

        Rc::new(Language {
            name: "julia".to_string(),
            exec: Box::new(julia::init),
            valid_args: Box::new(julia::valid_args),
            uses: string_set!["julia"],
            ignores: string_set![env::consts::OS, "julia"],
        }),

        Rc::new(Language {
            name: "kotlin".to_string(),
            exec: Box::new(kotlin::init),
            valid_args: Box::new(kotlin::valid_args),
            uses: string_set!["kotlin", "gradle"],
            ignores: string_set![env::consts::OS, "kotlin"],
        }),

        Rc::new(Language {
            name: "maven".to_string(),
            exec: Box::new(maven::init),
            valid_args: Box::new(maven::valid_args),
            uses: string_set!["maven", "java"],
            ignores: string_set![env::consts::OS, "maven", "java"],
        }),


        Rc::new(Language {
            name: "nextjs".to_string(),
            exec: Box::new(nextjs::init),
            valid_args: Box::new(nextjs::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "nextjs", "javascript", "typescript"],
        }),

        Rc::new(Language {
            name: "ocaml".to_string(),
            exec: Box::new(ocaml::init),
            valid_args: Box::new(ocaml::valid_args),
            uses: string_set!["ocaml", "opam"],
            ignores: string_set![env::consts::OS, "ocaml"],
        }),

        Rc::new(Language {
            name: "perl".to_string(),
            exec: Box::new(perl::init),
            valid_args: Box::new(perl::valid_args),
            uses: string_set!["perl"],
            ignores: string_set![env::consts::OS, "perl"],
        }),

        Rc::new(Language {
            name: "php".to_string(),
            exec: Box::new(php::init),
            valid_args: Box::new(php::valid_args),
            uses: string_set!["php", "laravel"],
            ignores: string_set![env::consts::OS, "php", "laravel"],
        }),

        Rc::new(Language {
            name: "python".to_string(),
            exec: Box::new(python::init),
            valid_args: Box::new(python::valid_args),
            uses: string_set!["python3"],
            ignores: string_set![env::consts::OS, "python"],
        }),

        Rc::new(Language {
            name: "reactjs".to_string(),
            exec: Box::new(reactjs::init),
            valid_args: Box::new(reactjs::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "react", "javascript"],
        }),

        Rc::new(Language {
            name: "reactts".to_string(),
            exec: Box::new(reactts::init),
            valid_args: Box::new(reactts::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "react", "javascript"],
        }),

        Rc::new(Language {
            name: "ruby".to_string(),
            exec: Box::new(ruby::init),
            valid_args: Box::new(ruby::valid_args),
            uses: string_set!["ruby", "rails"],
            ignores: string_set![env::consts::OS, "ruby", "rails"],
        }),

        Rc::new(Language {
            name: "rust".to_string(),
            exec: Box::new(rust::init),
            valid_args: Box::new(rust::valid_args),
            uses: string_set!["cargo"],
            ignores: string_set![env::consts::OS, "rust"],
        }),

        Rc::new(Language {
            name: "scala".to_string(),
            exec: Box::new(scala::init),
            valid_args: Box::new(scala::valid_args),
            uses: string_set!["scala", "gradle"],
            ignores: string_set![env::consts::OS, "scala"],
        }),

        Rc::new(Language {
            name: "svelte".to_string(),
            exec: Box::new(svelte::init),
            valid_args: Box::new(svelte::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "svelte"],
        }),

        Rc::new(Language {
            name: "swift".to_string(),
            exec: Box::new(swift::init),
            valid_args: Box::new(swift::valid_args),
            uses: string_set!["swift"],
            ignores: string_set![env::consts::OS, "swift"],
        }),

        Rc::new(Language {
            name: "typescript".to_string(),
            exec: Box::new(typescript::init),
            valid_args: Box::new(typescript::valid_args),
            uses: string_set!["typescript"],
            ignores: string_set![env::consts::OS, "typescript"],
        }),

        Rc::new(Language {
            name: "vue".to_string(),
            exec: Box::new(vue::init),
            valid_args: Box::new(vue::valid_args),
            uses: string_set!["npm"],
            ignores: string_set![env::consts::OS, "vue", "javascript", "typescript"],
        }),

        Rc::new(Language {
            name: "zig".to_string(),
            exec: Box::new(zig::init),
            valid_args: Box::new(zig::valid_args),
            uses: string_set!["zig"],
            ignores: string_set![env::consts::OS, "zig"],
        }),
    ];

    let mut out = HashMap::new();
    for lang in langs {
        out.insert(lang.name.clone(), lang);
    }
    out
}

// pub enum Language {
//     Python ( PythonFramework ),
//     Rust ( RustFramework ),
//     JavaScript ( JavaScriptFramework ),
//     TypeScript ( JavaScriptFramework ),
//     C,
//     Java,
//     Cpp,
//     Go,
//     Ruby,
//     Swift,
//     Kotlin,
//     Dart,
//     PHP,
//     Scala,
//     Elixir,
//     Erlang,
//     Crystal,
//     Nim,
//     Zig,
//     Lua,
//     Perl,
//     R,
//     Julia,
//     Haskell,
//     FSharp,
//     Clojure,
//     Groovy,
//     CSharp,
//     Unity,
//     Unreal,
//     Matlab,
// }
//
// enum PythonFramework {
//     Default,
//     Django,
//     Flask,
//     FastAPI,
// }
//
// enum RustFramework {
//     Default,
//     Actix,
//     Rocket,
//     Warp,
// }
//
// enum JavaScriptFramework {
//     Default,
//     Bootstrap,
//     Express,
//     Nodejs,
//     Nextjs,
//     React,
//     ReactNative,
//     Angular,
//     Vue,
//     Vite,
//     Svelte,
//     HTML5,
// }
