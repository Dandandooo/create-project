use std::env::args;
use create_project::*;
use args::*;
use dependencies::*;
use langs::*;

fn main() {
    let program = run();
    if let Err(err) = program {
        eprint!("{err}");
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn run() -> Result<(), Box<dyn Error>> {

    let globs = Globals {
        valid_global_args: global_args(),
        languages: supported_languages(),
        dependencies: dependencies(),
    }

    let cmd = Command::parse(args())?;
    cmd.exec(&global_args)?;

    Ok(())
}
