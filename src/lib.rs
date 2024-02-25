const NAME: &str = "hello-lang";

pub mod args;
pub mod langs;
pub mod dependencies;

pub type Res<T=()> = Result<T, Box<dyn Error>>;

use std::collections::{HashMap, HashSet};
use dependencies::Dependency;
use std::rc::Rc;
use std::error::Error;
use args::{ArgMap, ArgType, Arg, CommandConfig};
use langs::Language;

use crate::dependencies::ensure_installed;


#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = std::collections::HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

#[macro_export]
macro_rules! string_set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = std::collections::HashSet::new();
            $(
                temp_set.insert($x.to_string());
            )*
            temp_set
        }
    };
}

pub struct Globals {
    pub valid_global_args: ArgMap,
    pub languages: HashMap<String, Rc::<Language>>,
    pub dependencies: HashMap<String, Dependency>,
}

pub struct Command {
    pub global_config: CommandConfig,
    pub lang_config: Option<(Rc::<Language>, ArgMap, CommandConfig)>
}

impl Command {
    pub fn parse(args: std::env::Args, globs: &Globals) -> Res<Self> {
        let mut args = args.into_iter().skip(1).peekable();


        let mut global_config = CommandConfig {
            vars: HashMap::new(),
            flags: HashSet::new(),
        };

        let mut language: Option<String> = None;

        while let Some(arg) = args.next() {
            let next_arg = args.peek();
            let has_value = next_arg.map_or(false, |a| { a.starts_with("-") });

            if arg.starts_with("--") {
                let mut var = arg.chars().skip(2).collect::<String>();
                if has_value {
                    let arg = match globs.valid_global_args.get(&arg) {
                        Some(a) => a,
                        None => return Err(Box::from(format!("{} not a valid global option", arg)))
                    };

                    if let ArgType::Var { parse } = &arg.arg_type {
                        var = parse(&var)?;
                    }

                    global_config.vars.insert(var, args.next().unwrap());
                    continue;
                }

                if let None = globs.valid_global_args.get(&var) {
                    return Err(Box::from(format!("{} not a valid global flag", var)));
                }

                global_config.flags.insert(var);
                continue;
            } 

            if arg.starts_with("-") {
                let chars: Vec<char> = arg.chars().into_iter().skip(1).collect();

                if has_value {
                    if chars.len() > 1 {// this is invalid syntax
                        return Err(Box::from(format!(
                                "flags {} are not settable arguments\n
                                 run {} --help for a list of commands", 
                           &arg, NAME)));
                    }
                    let mut var = chars[0].to_string();
                    let arg = match globs.valid_global_args.get(&arg) {
                        Some(a) => a,
                        None => return Err(Box::from(format!("{} not a valid global option", arg)))
                    };

                    if let ArgType::Var { parse } = &arg.arg_type {
                        var = parse(&var)?;
                    }
                    global_config.vars.insert(var, args.next().unwrap());
                    global_config.vars.insert(chars[0].to_string(), args.next().unwrap());
                    continue;
                }

                for c in chars {
                    if let None = globs.valid_global_args.get(&c.to_string()) {
                        return Err(Box::from(format!("{} not a valid global flag", c)));
                    }
                    global_config.flags.insert(c.to_string());
                }

                continue;
            }

            language = Some(arg);
            break; // reached language
        }

        let mut lang_config = match language {
            Some(l_name) => Some({
                let lang = match globs.languages.get(&l_name) {
                    Some(l) => l.clone(),
                    None => return Err(Box::from(format!("
                            sorry, {} is not supported yet :(\n
                            run {} --list for a list of supported languages
                            ", l_name, NAME)))
                };
                let arg_map = (lang.valid_args)();
                let config = CommandConfig {
                    vars: HashMap::new(),
                    flags: HashSet::new(),
                };

                (lang, arg_map, config)
            }),
            None => None,
        };

        // process lang

        if let Some(lcfg) = &mut lang_config {
            while let Some(arg) = args.next() { // TODO change from global to lang

                let next_arg = args.peek();
                let has_value = next_arg.map_or(false, |a| { a.starts_with("-") });

                if arg.starts_with("--") {
                    let mut var = arg.chars().skip(2).collect::<String>();
                    if has_value {
                        let arg = match lcfg.1.get(&arg) {
                            Some(a) => a,
                            None => return Err(Box::from(format!("{} not a valid lang option", arg)))
                        };

                        if let ArgType::Var { parse } = &arg.arg_type {
                            var = parse(&var)?;
                        }

                        lcfg.2.vars.insert(var, args.next().unwrap());
                        continue;
                    }

                    if let None = lcfg.1.get(&var) {
                        return Err(Box::from(format!("{} not a valid lang flag", var)));
                    }

                    lcfg.2.flags.insert(var);
                    continue;
                } 

                if arg.starts_with("-") {
                    let chars: Vec<char> = arg.chars().into_iter().skip(1).collect();

                    if has_value {
                        if chars.len() > 1 {// this is invalid syntax
                            return Err(Box::from(format!(
                                        "flags {} are not settable arguments\n
                                 run {} --help for a list of commands", 
                                 &arg, NAME)));
                        }
                        let mut var = chars[0].to_string();
                        let arg = match lcfg.1.get(&arg) {
                            Some(a) => a,
                            None => return Err(Box::from(format!("{} not a valid lang option", arg)))
                        };

                        if let ArgType::Var { parse } = &arg.arg_type {
                            var = parse(&var)?;
                        }
                        lcfg.2.vars.insert(var, args.next().unwrap());
                        lcfg.2.vars.insert(chars[0].to_string(), args.next().unwrap());
                        continue;
                    }

                    for c in chars {
                        if let None = lcfg.1.get(&c.to_string()) {
                            return Err(Box::from(format!("{} not a valid lang flag", c)));
                        }
                        lcfg.2.flags.insert(c.to_string());
                    }

                    continue;
                }
            }
        }

        Ok(Self {
            global_config,
            lang_config,
        })
    }

    pub fn exec(self, globs: &Globals) -> Res {
        if let Some(lcfg) = &self.lang_config {
            self.global_command(globs)?;
            self.lang_command(globs)
        } else {
            self.global_command(globs)
        }
    }

    fn global_command(&self, globs: &Globals) -> Res {
        let flags = &self.global_config.flags;
        if flags.contains("help") {
            print_help(globs.valid_global_args);
            return Ok(());
        }
        
        if flags.contains("git") {
            ensure_installed(["git".to_string].iter(), &globs.dependencies);
        }

        if flags.contains()

        Ok(())
    }

    fn lang_command(self, globs: &Globals) -> Res {
        let lcfg = self.lang_config.unwrap();
        ensure_installed(lcfg.0.uses.iter(), &globs.dependencies)?;
        (lcfg.0.exec)(&lcfg.2)?;
        Ok(())
    }
}


