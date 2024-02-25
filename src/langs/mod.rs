mod js;   
mod ts;

mod rust;     

mod python;   

use super::args::{CommandConfig, ArgMap};
use std::collections::{HashSet, HashMap};
use super::string_set;

pub struct Language {
    pub name: String,
    pub exec: Box<dyn Fn(&CommandConfig) -> Result<(), String>>,
    pub valid_args: Box<dyn Fn() -> ArgMap>,
    pub uses: HashSet<String>, // dependencies
}

pub fn supported_languages() -> HashMap<String, Language> {
    let langs = [
        Language {
            name: "rust".to_string(),
            exec: Box::new(rust::init),
            valid_args: Box::new(rust::valid_args),
            uses: string_set!["cargo"],
        },
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
