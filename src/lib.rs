const NAME: &str = "create";

pub mod args;
pub mod langs;

use std::env::args;
use std::collections::{HashMap, HashSet};
use args::{ArgMap, Arg, CommandConfig};
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
    pub fn parse(args: std::env::Args) -> Result<Self, String> {
        let mut args = args.into_iter().skip(1).peekable();


        let mut global_config = CommandConfig {
            vars: HashMap::new(),
            flags: HashSet::new(),
        };

        while let Some(arg) = args.next() {
            let next_arg = args.peek();

            if arg.starts_with("--") {

            } else if arg.starts_with("-") {
                let chars = arg.chars().into_iter().skip(1).collect();
                let has_value = next_arg.map_or(false, |a| { a.starts_with("-") });
                if has_value && chars.size() > 1 { // this is invalid syntax
                    return Err(format!("flags {} are not settable arguments\n
                                       run {} --help for a list of commands", &arg, NAME));
                } 
                while let Some(c) = chars_iter.next(){

                }
            } else {
                break; // reached lang
            }
        }

        // process lang

        while let Some(arg) = args.next() {

        }

        Self {
            global_config,
            language,
            lang_config,
        }
    }

    fn validate(&self, valid_global_args: &ArgMap, valid_lang_args: &Option<ArgMap>) -> Result<(), String> {
        todo!()
    }

    pub fn exec(&self, valid_global_args: &ArgMap, supported_langs: &HashMap<String, Language>) -> Result<(), String>{
        let lang_args = match &self.language {
            Some(l) => {
                let args = supported_langs.get(l);
                match args {
                    Some(a) => Some((a.valid_args)()), 
                    None => return Err(format!("
                            sorry, {} is not supported yet :(\n
                            run {} --list for a list of supported languages
                            ", l, NAME))
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

    fn global_command(&self, valid_global_args: &ArgMap) -> Result<(), String> {
        todo!()
    }

    fn lang_command(&self, valid_global_args: &ArgMap, valid_lang_args: &ArgMap) -> Result<(), String> {
        todo!()
    }
}


