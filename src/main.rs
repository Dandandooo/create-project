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

fn run() -> Res {
    let globs = Globals {
        valid_global_args: global_args(),
        languages: supported_languages(),
        dependencies: dependencies()?,
    };
    ensure_installed(["cargo".to_string(), "upt".to_string()].iter(), &globs.dependencies)?;

    let cmd = Command::parse(args(), &globs)?;
    cmd.exec(&globs)?;

    Ok(())
}
