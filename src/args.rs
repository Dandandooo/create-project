use std::collections::{HashSet, HashMap}; 

pub const GLOBAL_ARGS: HashMap<String, &'static Arg> = hash_args(&GLOBAL_ARGS_LIST);

pub struct Arg {
    name: String, // with --
    description: String, // what will be printed in the help cmd
    aliases: HashSet<char>, // with -
    arg_type: ArgType, // flag or var
    mutually_exclusive: HashSet<String>, // other flags, vars
}

pub enum ArgType {
    Var {
        parse: Box<dyn Fn(String) -> Result<String, String> + Send + Sync>, // attempts to parse what the user has passed in
    },
    Flag,
}


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

pub static GLOBAL_ARGS_LIST: [Arg; 3] = [
    Arg {
        name: "name".to_string(),
        description: "the name of the project you want to make".to_string(),
        aliases: set!['n'],
        arg_type: ArgType::Var {
            parse: Box::new(|s| { Ok(s) }),
        },
        mutually_exclusive: HashSet::new(),
    },

    Arg {
        name: "git".to_string(),
        description: "initialises a git repository with ".to_string(),
        aliases: set!['g'],
        arg_type: ArgType::Flag,
        mutually_exclusive: set![],
    },

    Arg {
        name: "help".to_string(),
        description: "prints help info".to_string(),
        aliases: set!['h'],
        arg_type: ArgType::Flag,
        mutually_exclusive: string_set!["name", "git"],
    },
];

fn hash_args(args: &'static [Arg]) -> HashMap<String, &'static Arg> {
    let mut out = HashMap::new();
    for arg in args {
        out.insert(arg.name, arg);
        for alias in arg.aliases {
            out.insert(alias.to_string(), arg);
        }
    }
    return out;
}
