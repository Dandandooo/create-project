mod js;   
mod ts;

mod rust;     

mod python;   

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
    let langs = [
        Rc::new(Language {
            name: "rust".to_string(),
            exec: Box::new(rust::init),
            valid_args: Box::new(rust::valid_args),
            uses: string_set!["cargo"],
            ignores: string_set![env::consts::OS, "rust"],
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
