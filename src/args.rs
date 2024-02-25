use std::collections::{HashSet, HashMap}; 
use std::rc::Rc;

use super::{string_set};
use crate::Res;

pub type ArgMap = HashMap<String, Rc<Arg>>;

pub struct Arg {
    pub name: String, // with --
    pub description: String, // what will be printed in the help cmd
    pub aliases: HashSet<String>, // with -
    pub arg_type: ArgType, // flag or var
    pub mutually_exclusive: HashSet<String>, // other flags, vars
}

pub enum ArgType {
    Var {
        parse: Box<dyn Fn(&str) -> Res<String>>, // attempts to parse what the user has passed in
    },
    Flag,
}

pub struct CommandConfig {
    pub vars: HashMap<String, String>,
    pub flags: HashSet<String>,
}


pub fn global_args() -> ArgMap {
    let args = [
        Rc::new(Arg {
            name: "name".to_string(),
            description: "the name of the project you want to make".to_string(),
            aliases: string_set!["n"],
            arg_type: ArgType::Var {
                parse: Box::new(|s| { Ok(s.to_string()) }),
            },
            mutually_exclusive: HashSet::new(),
        }),

        Rc::new(Arg {
            name: "git".to_string(),
            description: "initialises a git repository with ".to_string(),
            aliases: string_set!["g"],
            arg_type: ArgType::Flag,
            mutually_exclusive: HashSet::new(),
        }),

        Rc::new(Arg {
            name: "help".to_string(),
            description: "prints help info".to_string(),
            aliases: string_set!["h"],
            arg_type: ArgType::Flag,
            mutually_exclusive: string_set!["name", "git"],
        }),

        Rc::new(Arg {
            name: "version".to_string(),
            description: "prints current version".to_string(),
            aliases: string_set!["v"],
            arg_type: ArgType::Flag,
            mutually_exclusive: string_set!["name", "git"],
        }),
    ];
    let mut out = HashMap::new();
    for arg in args {
        out.insert(arg.name.clone(), arg.clone());
        for alias in &arg.aliases {
            out.insert(alias.clone(), arg.clone());
        }
    }
    out
}
