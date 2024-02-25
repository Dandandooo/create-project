const NAME: &str = "create";

pub mod args;
pub mod langs;

pub type Res<T=()> = Result<T, Box<dyn Error>>;

use std::env::args;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use args::{ArgMap, ArgType, Arg, CommandConfig};
use langs::Language;


#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
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
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x.to_string());
            )*
            temp_set
        }
    };
}

pub struct Command {
    pub global_config: CommandConfig,
    pub language: Option<String>,
    pub lang_config: Option<CommandConfig>,
}

impl Command {
    pub fn parse(args: std::env::Args) -> Res<Self> {
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
                if has_value {
                    global_config.vars.insert(arg, args.next().unwrap());
                    continue;
                }

                global_config.flags.insert(arg);
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
                    global_config.vars.insert(chars[0].to_string(), args.next().unwrap());
                    continue;
                }

                for c in chars {
                    global_config.flags.insert(c.to_string());
                }

                continue;
            }

            language = Some(arg);
            break; // reached language
        }

        let mut lang_config = language.map(|_| {
            CommandConfig {
                vars: HashMap::new(),
                flags: HashSet::new(),
            }
        });

        // process lang

        while let Some(arg) = args.next() {
            let mut lang_config = &lang_config.unwrap();
            let next_arg = args.peek();
            let has_value = next_arg.map_or(false, |a| { a.starts_with("-") });

            if arg.starts_with("--") {
                if has_value {
                    global_config.vars.insert(arg, args.next().unwrap());
                    continue;
                }

                global_config.flags.insert(arg);
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
                    global_config.vars.insert(chars[0].to_string(), args.next().unwrap());
                    continue;
                }

                for c in chars {
                    global_config.flags.insert(c.to_string());
                }

                continue;
            }
        }

        Ok(Self {
            global_config,
            language,
            lang_config,
        })
    }

    fn validate(&mut self, valid_global_args: &ArgMap, valid_lang_args: &Option<ArgMap>) -> Res {
        for flag in &self.global_config.flags {
            if let None = valid_global_args.get(flag) {
                return Err(Box::from(format!("{} not a valid global flag", flag)));
            }
        }

        for (var, val) in &mut self.global_config.vars {
            let arg = match valid_global_args.get(var) {
                Some(a) => a,
                None => return Err(Box::from(format!("{} not a valid global option", var)))
            };

            if let ArgType::Var { parse } = arg.arg_type {
                *val = parse(&val)?;
            }
        }

        if let (Some(l), Some(v)) = (&self.lang_config, &valid_lang_args) {
            for flag in &l.flags {
                if let None = v.get(flag) {
                    return Err(Box::from(format!("{} not a valid flag for {}", flag, self.language.unwrap())));
                }
            }
        }
        Ok(())
    }

    pub fn exec(&mut self, valid_global_args: &ArgMap, supported_langs: &HashMap<String, Language>) -> Res{
        let lang_args = match &self.language {
            Some(l) => {
                let args = supported_langs.get(l);
                match args {
                    Some(a) => Some((a.valid_args)()), 
                    None => return Err(Box::from(format!("
                            sorry, {} is not supported yet :(\n
                            run {} --list for a list of supported languages
                            ", l, NAME)))
                }
            }, 
            None => None,
        };
        self.validate(valid_global_args, &lang_args)?;


        if let Some(lang_args) = lang_args {
            self.global_command(valid_global_args);
            self.lang_command(valid_global_args, &lang_args)
        } else {
            self.global_command(valid_global_args)
        }
    }

    fn global_command(&self, valid_global_args: &ArgMap) -> Res {
        todo!()
    }

    fn lang_command(&self, valid_global_args: &ArgMap, valid_lang_args: &ArgMap) -> Res {
        todo!()
    }
}


